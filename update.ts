import * as log from "https://deno.land/std@0.220.1/log/mod.ts";
import * as toml from "https://deno.land/std@0.220.1/toml/mod.ts";
import * as semver from "https://deno.land/std@0.220.1/semver/mod.ts";

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
    return `https://www.keycloak.org/docs-api/${version}/rest-api/openapi.json`;
  }

  async apiOpenJson(version: Version): Promise<string> {
    const response = await fetch(this.apiUrl(version));
    return await responseBody(response);
  }
}

const EXAMPLE_FOR_GENERATION = "openapi";

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

  async cleanKeycloak() {
    return await this.cargoCommandSpawn(["clean", "-p", "keycloak"]);
  }

  async build() {
    return await this.cargoCommandSpawn(["build"]);
  }

  async publish() {
    return await this.cargoCommandSpawn(["publish"]);
  }

  async fmt() {
    return await this.cargoCommand(["fmt"]);
  }

  async generateTypes(): Promise<string> {
    return await this.generate("types");
  }

  async generateRest(): Promise<string> {
    return await this.generate("rest");
  }

  private async generate(kind: "rest" | "types"): Promise<string> {
    return await this.cargoCommand([
      "run",
      "--example",
      EXAMPLE_FOR_GENERATION,
      "--",
      kind,
    ]);
  }

  private async cargoCommand(args: string[]): Promise<string> {
    return await Command.execute("cargo", args, "inherit");
  }

  private async cargoCommandSpawn(args: string[]): Promise<void> {
    await Command.spawn("cargo", args);
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
    const { mode } = await prompt([
      {
        name: "mode",
        message: "Type of update",
        type: Select,
        options: [
          {
            name: "Generation",
            value: "generation",
          },
          {
            name: "Release",
            value: "release",
          },
        ],
      },
    ]);
    const release = mode === "release";
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
              break;
            default:
              milestoneVersion = new InternalVersion(baseVersion!);
              break;
          }
          options.milestoneVersion = milestoneVersion;
          await next();
        },
      },
      {
        name: "enterMilestone",
        message: "Enter Milestone",
        type: Input,
        before: async (_, next) => {
          if (options.milestoneVersion) {
            await next("changeCargoTomlVersion");
          } else {
            await next();
          }
        },
        after: async ({ enterMilestone }, next) => {
          const milestoneVersion = new InternalVersion(enterMilestone!);
          options.milestoneVersion = milestoneVersion;
          await next();
        },
      },
      {
        name: "changeCargoTomlVersion",
        message: `Change Cargo.toml version?`,
        type: Confirm,
        default: true,
        before: async ({ baseVersion }, next) => {
          if (baseVersion === "cargo") {
            await next("downloadApi");
          } else {
            await next();
          }
        },
      },
      {
        name: "downloadApi",
        message: `Download API description?`,
        type: Confirm,
        default: true,
      },
      {
        name: "updateDocs",
        message: `Update documentation?`,
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
        name: "createMilestone",
        message: `Milestone does not exist. Should we create it?`,
        type: Confirm,
        default: release,
        before: async (_, next) => {
          const milestone = await updater.git.milestone(
            options.milestoneVersion!.toString(),
          );
          if (!milestone) {
            await next();
          } else {
            versions.milestone = milestone;
            await next("assignMilestone");
          }
        },
      },
      {
        name: "assignMilestone",
        message: `Assign milestone to issues and merge requests?`,
        type: Confirm,
        default: release,
        before: async (createMilestone, next) => {
          if (versions.milestone || createMilestone) {
            await next();
          } else {
            await next("createReleaseIssue");
          }
        },
      },
      {
        name: "createReleaseIssue",
        message: `Create release issue?`,
        type: Confirm,
        default: release,
      },
      {
        name: "gitCommit",
        message: `Create commit in Git?`,
        type: Confirm,
        default: release,
      },
      {
        name: "gitTag",
        message: `Create tag in Git?`,
        type: Confirm,
        default: release,
      },
      {
        name: "gitPush",
        message: `Push changes to GitHub?`,
        type: Confirm,
        default: release,
      },
      {
        name: "createReleasePullRequest",
        message: `Create release pull request?`,
        type: Confirm,
        default: release,
      },
      {
        name: "mergeReleasePullRequest",
        message: `Merge release pull request?`,
        type: Confirm,
        default: release,
      },
      {
        name: "gitRelease",
        message: `Create release on GitHub?`,
        type: Confirm,
        default: release,
      },
      {
        name: "cratesPublish",
        message: `Publish release on crates.io?`,
        type: Confirm,
        default: release,
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
  static async spawn(
    cmd: string,
    args: string[],
  ): Promise<void> {
    const command = new Deno.Command(cmd, { args });

    const child = await command.spawn();
    const code = (await child.status).code;
    if (code !== 0) {
      throw new CommandError(`Error ${code}`);
    }
  }

  static async execute(
    cmd: string,
    args: string[],
    stderr: "piped" | "inherit" = "piped",
  ): Promise<string> {
    const command = new Deno.Command(cmd, {
      args,
      stdout: "piped",
      stderr,
    });

    const output = await command.output();
    if (output.code !== 0) {
      if (stderr == "piped") {
        throw new CommandError(new TextDecoder().decode(output.stderr));
      } else {
        throw new CommandError("failed");
      }
    }
    return new TextDecoder().decode(output.stdout);
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

  async checkout(args: string[]) {
    await this.gitCommand(["checkout", ...args]);
  }

  async commit(options: { message: string }) {
    await this.gitCommand([
      "commit",
      "-a",
      "--allow-empty",
      "-m",
      options.message,
    ]);
  }

  async tag(options: { code: string; message: string }) {
    await this.gitCommand(["tag", "-f", "-m", options.message, options.code]);
  }

  async push(args: string[] = []) {
    await this.gitCommand(["push", ...args]);
  }

  async stash(args: string[] = []) {
    await this.gitCommand(["stash", ...args]);
  }

  async createRelease(options: { title: string; version: string }) {
    await this.ghCommand([
      "release",
      "create",
      options.version,
      "--title",
      options.title,
      "--generate-notes",
    ]);
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

    const issueNumber = githubNumberFromUrl(issueUrl.trim());

    return await this.issue(issueNumber!);
  }

  async developIssue(issue: Issue): Promise<void> {
    await this.push();
    await this.stash();
    await this.ghCommand([
      "issue",
      "develop",
      issue.number.toString(),
      "--checkout",
    ]);
    await this.stash(["pop"]);
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

  async createPullRequest(
    options: { title: string; milestone: Milestone },
  ): Promise<PullRequest> {
    const pullRequestUrl: string = await this.ghCommand([
      "pr",
      "create",
      "--assignee",
      "@me",
      "--milestone",
      options.milestone.title,
      "--title",
      options.title,
      "--fill",
    ]);

    const pullRequestNumber = githubNumberFromUrl(pullRequestUrl.trim());

    return await this.pullRequest(pullRequestNumber!);
  }

  async mergePullRequest() {
    await this.ghCommand([
      "pr",
      "merge",
      "-s",
      "-d",
      "--admin",
    ]);
  }

  async pullRequest(number: string): Promise<PullRequest> {
    return await this.ghCommandJson([
      "pr",
      "view",
      number,
      "--json",
      "number,milestone,state,title",
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

  get majorVersion(): string {
    const { major, minor } = this.value;
    return `${major}.${minor}`;
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
  pullRequest: PullRequest;

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
    const version = milestoneVersion.toVersion();
    const majorVersion = version.majorVersion;

    if (options.mergeReleasePullRequest) {
      options.gitPush = true;
    }

    Deno.env.set("KEYCLOAK_RUST_VERSION", milestoneVersion.toString());
    Deno.env.set("KEYCLOAK_VERSION", version.toString());
    Deno.env.set("KEYCLOAK_RUST_MAJOR_VERSION", majorVersion);

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

    if (options.downloadApi) {
      const api = await this.keycloak.apiOpenJson(milestoneVersion.toVersion());
      Deno.writeTextFileSync("api/openapi.json", api);
    }

    if (options.updateDocs) {
      await Command.spawn("handlebars-magic", ["templates", "."]);
    }

    if (options.runGenerator) {
      this.info("Cleaning old types and rest...");
      await this.git.checkout([
        "--",
        "src/types.rs",
        "src/rest/generated_rest.rs",
      ]);

      this.info("Generating new...");

      const codeTypes = await this.cargo.generateTypes();
      const codeRest = await this.cargo.generateRest();

      Deno.writeTextFileSync("src/types.rs", codeTypes);
      Deno.writeTextFileSync("src/rest/generated_rest.rs", codeRest);

      this.info("Formatting...");

      await this.cargo.fmt();

      this.info("Building...");
      await this.cargo.build();
    }

    if (options.createReleaseIssue && milestoneExists) {
      const issue = await this.createReleaseIssue(milestoneVersion);
      this.options.versions.issue = issue;
      if (isDefaultBranch) {
        await this.git.developIssue(issue);
      }
    }

    if (options.gitCommit) {
      await this.git.commit({
        message: `Keycloak Admin REST API v${milestoneVersion.toString()}`,
      });
    }

    if (options.gitTag) {
      await this.git.tag({
        code: `v${milestoneVersion.toString()}`,
        message: `Keycloak Admin REST API v${milestoneVersion.toString()}`,
      });
    }

    if (options.gitPush) {
      await this.git.push();
      await this.git.push(["--tag"]);
    }

    if (options.createReleasePullRequest) {
      let issueExists = this.options.versions.issue !== undefined;
      if (!issueExists) {
        const number = detectExistingIssue(currentBranch.toString());
        if (number) {
          this.options.versions.issue = await this.git.issue(number);
          issueExists = true;
          const pullRequest =
            (await this.git.pullRequests(`head:${currentBranch}`)).pop();
          if (
            pullRequest !== undefined &&
            pullRequest.milestone !== milestoneVersion.toString()
          ) {
            this.info(
              `Pull request already exists, but milestone is different (${pullRequest.milestone}), assigning new milestone.`,
            );
            await this.git.setPullRequestMilestone(
              pullRequest,
              milestoneVersion,
            );
            this.options.versions.pullRequest = pullRequest;
          }
        }
      }

      if (issueExists && this.options.versions.pullRequest === undefined) {
        this.options.versions.pullRequest = await this.createReleasePullRequest(
          milestoneVersion,
        );
      } else {
        this.info(`Could not create release pull request: no issue detected`);
      }
    }

    if (options.mergeReleasePullRequest) {
      await this.mergePullRequest();
    }

    if (options.gitRelease) {
      const releaseVersion = `v${milestoneVersion.toString()}`;
      await this.git.createRelease({
        title: releaseVersion,
        version: releaseVersion,
      });
    }

    if (options.cratesPublish) {
      await this.cargo.publish();
    }
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

    return await this.git.createIssue({
      title: `Release v${milestoneVersion}`,
      milestone: this.options.versions.milestone!,
      body,
    });
  }

  private async createReleasePullRequest(
    milestoneVersion: InternalVersion,
  ): Promise<PullRequest> {
    this.info(`Creating release pull request...`);
    return await this.git.createPullRequest({
      title: `Release v${milestoneVersion}`,
      milestone: this.options.versions.milestone!,
    });
  }

  private async mergePullRequest() {
    this.info(`Merging pull request...`);
    await this.git.mergePullRequest();
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

function githubNumberFromUrl(issueUrl: string): string | undefined {
  return issueUrl.match(/\d+$/)?.pop();
}

async function main(_args: string[]) {
  await new Updater().run();
}

main(Deno.args);

import { assertEquals } from "https://deno.land/std@0.220.1/assert/mod.ts";

Deno.test("detectExistingIssue", () => {
  assertEquals(
    detectExistingIssue("62-allow-release"),
    "62",
    "cannot detect existing issue from branch",
  );
});

Deno.test("issueNumberFromUrl", () => {
  assertEquals(
    githubNumberFromUrl("http://github.com/owner/repo/issues/123"),
    "123",
    "cannot detect existing issue from url",
  );
});
