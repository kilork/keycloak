import * as log from "https://deno.land/std@0.214.0/log/mod.ts";
import * as toml from "https://deno.land/std@0.214.0/toml/mod.ts";
import * as semver from "https://deno.land/std@0.214.0/semver/mod.ts";

import {
  Checkbox,
  Confirm,
  Input,
  prompt,
  Select,
} from "https://deno.land/x/cliffy@v1.0.0-rc.3/prompt/mod.ts";

class HttpError extends Error {
  readonly body?: string;

  constructor(message: string, body?: string) {
    super(message);
    this.body = body;
    Object.setPrototypeOf(this, HttpError.prototype);
  }
}

class CommandError extends Error {
  constructor(message: string) {
    super(message);
    Object.setPrototypeOf(this, CommandError.prototype);
  }
}

async function responseBody(response: Response): Promise<string> {
  if (!response.ok) {
    const message = `${response.status} ${response.statusText}`;
    let body;

    if (response.bodyUsed) {
      body = await response.text();
    }

    throw new HttpError(message, body);
  }
  return await response.text();
}

class Keycloak {
  async latestVersion(): Promise<Version> {
    const response = await fetch("https://keycloak.org");
    const body = await responseBody(response);

    return new Version((/Latest release (.+)\s/.exec(body) ?? [])[1]);
  }

  apiUrl(version: Version): string {
    return `https://www.keycloak.org/docs-api/${version}/rest-api/index.html`;
  }

  async apiDocs(version: Version): Promise<string> {
    const response = await fetch(this.apiUrl(version));
    return await responseBody(response);
  }
}

class Cargo {
  get text(): string {
    return Deno.readTextFileSync("Cargo.toml");
  }

  set text(value) {
    Deno.writeTextFileSync("Cargo.toml", value);
  }

  get version(): InternalVersion {
    const cargoToml = toml.parse(this.text);

    interface Package {
      readonly version: string;
    }

    return new InternalVersion((cargoToml.package as Package).version);
  }

  set version(value: InternalVersion) {
    const lines = this.text.split("\n");
    const index = lines.findIndex((line) => line.startsWith("version = "));
    if (index !== -1) {
      lines[index] = `version = "${value}"`;
      this.text = lines.join("\n");
    }
  }
}

class Branch {
  readonly value: string;

  constructor(value: string) {
    this.value = value.trim();
  }

  toString() {
    return this.value;
  }

  equals(other: Branch): boolean {
    return other.value === this.value;
  }
}

type IssueState = "CLOSED" | "OPEN";
type PullRequestState = "CLOSED" | "OPEN" | "MERGED";

interface Issue {
  milestone?: string;
  number: number;
  state: IssueState;
  title: string;
}

interface Milestone {
  number: number;
  title: string;
}

interface PullRequest {
  milestone?: string;
  number: number;
  state: PullRequestState;
  title: string;
}

interface IssueOrPullRequest {
  issue?: Issue;
  pullRequest?: PullRequest;
}

class User {
  autoconfirm: boolean;

  constructor(autoconfirm: boolean = false) {
    this.autoconfirm = autoconfirm;
  }

  async confirm(message: string): Promise<boolean> {
    if (this.autoconfirm) {
      return true;
    }

    return await Confirm.prompt(message);
  }

  async selectUpdateOptions(updater: Updater) {
    const { options } = updater;
    const { versions } = options;
    const latestVersion = versions.keycloakLatestVersion?.toInternalVersion();
    const cargoVersion = versions.keycloakCargoVersion;
    const result = await prompt([
      {
        name: "baseVersion",
        message: "Select which version to use for update",
        type: Select,
        options: [
          {
            name: `Latest Keycloak version: ${latestVersion}`,
            value: "latest",
          },
          {
            name: `Cargo.toml version: ${cargoVersion}`,
            value: "cargo",
          },
          ...versions.currentMilestones!.map((milestone) => ({
            name: `Milestone ${milestone.title}`,
            value: milestone.title,
          })),
          ...[
            {
              name: `Enter manually`,
              value: "manual",
            },
          ],
        ],
        after: async ({ baseVersion }, next) => {
          let milestoneVersion;
          switch (baseVersion) {
            case "latest":
              milestoneVersion = latestVersion;
              break;
            case "cargo":
              milestoneVersion = cargoVersion;
              break;
            case "manual":
              await next("enterMilestone");
              return;
            default:
              milestoneVersion = new InternalVersion(baseVersion!);
              break;
          }
          options.milestoneVersion = milestoneVersion;
          const milestone = await updater.git.milestone(
            milestoneVersion!.toString(),
          );
          if (!milestone) {
            await next("createMilestone");
          } else {
            versions.milestone = milestone;
            await next("assignMilestone");
          }
        },
      },
      {
        name: "enterMilestone",
        message: "Enter Milestone",
        type: Input,
        after: async ({ enterMilestone }, next) => {
          const milestoneVersion = new InternalVersion(enterMilestone!);
          options.milestoneVersion = milestoneVersion;
          const milestone = await updater.git.milestone(
            milestoneVersion!.toString(),
          );
          if (!milestone) {
            await next("createMilestone");
          } else {
            versions.milestone = milestone;
            await next("assignMilestone");
          }
        },
      },
      {
        name: "createMilestone",
        message: `Milestone does not exist. Should we create it?`,
        type: Confirm,
        default: true,
      },
      {
        name: "assignMilestone",
        message: `Assign milestone to issues and merge requests?`,
        type: Confirm,
        default: true,
        before: async ({ createMilestone }, next) => {
          if (!(createMilestone ?? true)) {
            await next("changeCargoTomlVersion");
          } else {
            await next();
          }
        },
      },
      {
        name: "changeCargoTomlVersion",
        message: `Change Cargo.toml version?`,
        type: Confirm,
        default: true,
        before: async ({ baseVersion }, next) => {
          if (baseVersion === "cargo") {
            await next("downloadApiDocs");
          } else {
            await next();
          }
        },
      },
      {
        name: "downloadApiDocs",
        message: `Download API documentation?`,
        type: Confirm,
        default: true,
      },
      {
        name: "runGenerator",
        message: `Run generator?`,
        type: Confirm,
        default: true,
      },
      {
        name: "createReleaseIssue",
        message: `Create release issue?`,
        type: Confirm,
        default: true,
      },
      {
        name: "gitCommit",
        message: `Create commit in Git?`,
        type: Confirm,
        default: true,
      },
      {
        name: "gitTag",
        message: `Create tag in Git?`,
        type: Confirm,
        default: true,
      },
      {
        name: "gitPush",
        message: `Push changes to GitHub?`,
        type: Confirm,
        default: true,
      },
      {
        name: "createReleasePullRequest",
        message: `Create release pull request?`,
        type: Confirm,
        default: true,
      },
      {
        name: "gitRelease",
        message: `Create release on GitHub?`,
        type: Confirm,
        default: true,
      },
      {
        name: "cratesPublish",
        message: `Publish release on crates.io?`,
        type: Confirm,
        default: true,
      },
    ]);

    return result as Required<typeof result>;
  }

  async selectIssuesAndPullRequests(
    issues: Issue[],
    pullRequests: PullRequest[],
  ): Promise<IssueOrPullRequest[]> {
    return (
      await Checkbox.prompt({
        message: "Select issues and pull requests to assign milestone",
        options: [
          {
            name: "Issues",
            options: issues.map((issue) => ({
              name: `#${issue.number} ${issue.title} (${issue.state})`,
              value: { issue },
            })),
          },
          {
            name: "Pull requests",
            options: pullRequests.map((pullRequest) => ({
              name:
                `#${pullRequest.number} ${pullRequest.title} (${pullRequest.state})`,
              value: { pullRequest },
            })),
          },
        ],
      })
    ).map((item) => item as IssueOrPullRequest);
  }
}

class Command {
  static async execute(cmd: string, args: string[]): Promise<string> {
    const command = new Deno.Command(cmd, {
      args,
      stdout: "piped",
    });

    const { code, stdout, stderr } = await command.output();
    if (code !== 0) {
      throw new CommandError(new TextDecoder().decode(stderr));
    }
    return new TextDecoder().decode(stdout);
  }
}

class Git {
  private readonly repository;

  constructor(repository: string) {
    this.repository = repository;
  }

  async currentBranch(): Promise<Branch> {
    return new Branch(await this.gitCommand(["branch", "--show-current"]));
  }

  async defaultBranch(): Promise<Branch> {
    const prefix = "refs/remotes/origin/";
    const refs = await this.gitCommand(["symbolic-ref", `${prefix}HEAD`]);
    if (refs.startsWith(prefix)) {
      return new Branch(refs.substring(prefix.length));
    } else {
      throw new CommandError(
        `could not determine default branch, output: ${refs}`,
      );
    }
  }

  async issue(number: string): Promise<Issue> {
    return await this.ghCommandJson([
      "issue",
      "view",
      number,
      "--json",
      "number,milestone,title",
    ]);
  }

  async issues(
    search:
      | string
      | undefined = undefined,
  ): Promise<Issue[]> {
    return await this.ghCommandJson([
      "issue",
      "list",
      "-s",
      "all",
      "--json",
      "number,milestone,title,state",
      "--search",
      search ?? "",
    ]);
  }

  async issuesNoMilestone(): Promise<Issue[]> {
    return await this.issues("no:milestone");
  }

  async createIssue(
    options: { title: string; body: string; milestone: Milestone },
  ): Promise<Issue> {
    const issueUrl: string = await this.ghCommand([
      "issue",
      "create",
      "--assignee",
      "@me",
      "--milestone",
      options.milestone.title,
      "--title",
      options.title,
      "--body",
      options.body,
    ]);

    const issueNumber = issueNumberFromUrl(issueUrl.trim());

    return await this.issue(issueNumber!);
  }

  async developIssue(issue: Issue): Promise<void> {
    await this.ghCommand([
      "issue",
      "develop",
      issue.number.toString(),
      "--checkout",
    ]);
  }

  async setIssueMilestone(issue: Issue, milestoneVersion: InternalVersion) {
    await this.ghCommand([
      "issue",
      "edit",
      issue.number.toString(),
      "--milestone",
      milestoneVersion.toString(),
    ]);
  }

  async milestone(title: string): Promise<Milestone | undefined> {
    return (await this.milestones("all")).find((m) => m.title === title);
  }

  async milestones(state: "all" | "open"): Promise<Milestone[]> {
    return await this.ghCommandJson([
      "api",
      "-H",
      "Accept: application/vnd.github.v3+json",
      `/repos/${this.repository}/milestones?state=${state}`,
    ]);
  }

  async createMilestone(version: InternalVersion): Promise<Milestone> {
    return await this.ghCommandJson([
      "api",
      "--method",
      "POST",
      "-H",
      "Accept: application/vnd.github.v3+json",
      `/repos/${this.repository}/milestones`,
      "-f",
      `title=${version}`,
      "-f",
      "state=open",
    ]);
  }

  async pullRequests(
    search:
      | string
      | undefined = undefined,
  ): Promise<PullRequest[]> {
    return await this.ghCommandJson([
      "pr",
      "list",
      "-s",
      "all",
      "--json",
      "number,milestone,title,state",
      "--search",
      search ?? "",
    ]);
  }

  async pullRequestsNoMilestone(): Promise<PullRequest[]> {
    return await this.pullRequests("no:milestone");
  }

  async setPullRequestMilestone(
    pullRequest: PullRequest,
    milestoneVersion: InternalVersion,
  ) {
    await this.ghCommand([
      "pr",
      "edit",
      pullRequest.number.toString(),
      "--milestone",
      milestoneVersion.toString(),
    ]);
  }

  private async gitCommand(args: string[]): Promise<string> {
    return await Command.execute("git", args);
  }

  private async ghCommand(args: string[]): Promise<string> {
    return await Command.execute("gh", args);
  }

  private async ghCommandJson<T>(args: string[]): Promise<T> {
    return JSON.parse(await this.ghCommand(args));
  }
}

class Version {
  readonly asString: string;
  readonly value: semver.SemVer;

  constructor(str: string) {
    this.asString = str;
    this.value = semver.parse(str);
  }

  toString() {
    return this.asString;
  }

  toInternalVersion(): InternalVersion {
    const { major, minor, patch } = this.value;
    return new InternalVersion(`${major}.${minor}.${Math.round(patch * 100)}`);
  }
}

class InternalVersion {
  private version: Version;
  constructor(str: string) {
    this.version = new Version(str);
  }

  toString() {
    return this.version.toString();
  }

  toVersion(): Version {
    const { major, minor, patch } = this.version.value;
    return new Version(`${major}.${minor}.${Math.round(patch / 100)}`);
  }

  get fixVersion(): boolean {
    return this.version.value.patch % 100 !== 0;
  }
}

class Options {
  milestoneVersion?: InternalVersion;
  versions: Partial<Versions> = {};

  toString(): string {
    return JSON.stringify(this, null, 2);
  }
}

interface Versions {
  milestone: Milestone;
  pr: string;
  issue: Issue;

  currentBranch: Branch;
  defaultBranch: Branch;
  currentMilestones: Milestone[];

  keycloakLatestVersion: Version;
  keycloakCargoVersion: InternalVersion;
}

class Updater {
  readonly cargo: Cargo = new Cargo();
  readonly git: Git = new Git("kilork/keycloak");
  readonly keycloak: Keycloak = new Keycloak();
  readonly user: User = new User();

  options: Options = new Options();

  async run() {
    const currentBranch = await this.git.currentBranch();
    const defaultBranch = await this.git.defaultBranch();
    const versions = {
      keycloakLatestVersion: await this.keycloak.latestVersion(),
      keycloakCargoVersion: this.cargo.version,
      currentBranch,
      defaultBranch,
      currentMilestones: (await this.git.milestones("open")).filter(
        (milestone) => milestone.title !== "released",
      ),
    };

    this.options.versions = versions;

    const options = await this.user.selectUpdateOptions(this);

    const milestoneVersion = this.options.milestoneVersion!;

    if (options.createMilestone) {
      await this.createMilestone(milestoneVersion);
    }

    const milestoneExists = this.options.versions.milestone !== undefined;

    if (options.assignMilestone) {
      await this.assignMilestone(milestoneVersion);
    }

    if (options.baseVersion !== "cargo" && options.changeCargoTomlVersion) {
      this.changeCargoTomlVersion(milestoneVersion);
    }

    const isDefaultBranch = versions.currentBranch.equals(
      versions.defaultBranch,
    );
    if (options.createReleaseIssue && milestoneExists) {
      const issue = await this.createReleaseIssue(milestoneVersion);
      this.options.versions.issue = issue;
      if (isDefaultBranch) {
        await this.git.developIssue(issue);
      }
    }

    if (options.createReleasePullRequest) {
      const issueExists = this.options.versions.issue !== undefined;
      if (!issueExists) {
        const number = detectExistingIssue(
          this.options.versions.currentBranch!.toString(),
        );
        if (number) {
          this.options.versions.issue = await this.git.issue(number);
          const pullRequest =
            (await this.git.pullRequests(`head:${currentBranch}`)).pop();
          if (pullRequest !== undefined) {
          }
        }
      }

      if (issueExists) {
      } else {
        this.info(`Could not create release pull request: no issue detected`);
      }
    }
    // this.info(`${this.options}`);

    // this.info(keycloakApiDocs);
  }

  private async assignMilestone(milestoneVersion: InternalVersion) {
    const issues = await this.git.issuesNoMilestone();
    const pullRequests = await this.git.pullRequestsNoMilestone();

    const selected = await this.user.selectIssuesAndPullRequests(
      issues,
      pullRequests,
    );

    for (const item of selected) {
      if (item.issue !== undefined) {
        const issue = item.issue;
        this.info(
          `Changing issue #${issue.number} milestone to ${milestoneVersion}...`,
        );
        await this.git.setIssueMilestone(issue, milestoneVersion);
      } else if (item.pullRequest !== undefined) {
        const pullRequest = item.pullRequest;
        this.info(
          `Changing pull request #${pullRequest.number} milestone to ${milestoneVersion}...`,
        );
        await this.git.setPullRequestMilestone(
          item.pullRequest,
          milestoneVersion,
        );
      }
    }
  }

  private changeCargoTomlVersion(milestoneVersion: InternalVersion) {
    this.info(`Changing Cargo.toml version to ${milestoneVersion}...`);
    this.cargo.version = milestoneVersion;
  }

  private async createReleaseIssue(
    milestoneVersion: InternalVersion,
  ): Promise<Issue> {
    this.info(`Creating release issue...`);
    let body;
    if (milestoneVersion.fixVersion) {
      body = `Patch release`;
    } else {
      body =
        `There is a new version of [keycloak](https://www.keycloak.org/) API:

- ${this.keycloak.apiUrl(milestoneVersion.toVersion())}
        `;
    }
    const issue = await this.git.createIssue({
      title: `Release v${milestoneVersion}`,
      milestone: this.options.versions.milestone!,
      body,
    });

    return issue;
  }

  private async createMilestone(milestoneVersion: InternalVersion) {
    this.info(`Creating milestone ${milestoneVersion}...`);
    const milestone = await this.git.createMilestone(milestoneVersion);

    this.options.versions.milestone = milestone;
  }

  info(message: string) {
    log.info(message);
  }

  error(message: string) {
    log.error(message);
  }
}

function detectExistingIssue(currentBranch: string): string | undefined {
  return currentBranch.match(/^\d+/)?.pop();
}

function issueNumberFromUrl(issueUrl: string): string | undefined {
  return issueUrl.match(/\d+$/)?.pop();
}

async function main(_args: string[]) {
  await new Updater().run();
}

main(Deno.args);

import { assertEquals } from "https://deno.land/std@0.214.0/assert/mod.ts";

Deno.test("detectExistingIssue", () => {
  assertEquals(
    detectExistingIssue("62-allow-release"),
    "62",
    "cannot detect existing issue from branch",
  );
});

Deno.test("issueNumberFromUrl", () => {
  assertEquals(
    issueNumberFromUrl("http://github.com/owner/repo/issues/123"),
    "123",
    "cannot detect existing issue from url",
  );
});
