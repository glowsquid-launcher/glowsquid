import { Adapter } from "./types";
import { Octokit } from "octokit";
import { promises as fs } from "fs";
import fetch from "node-fetch";

export let sourcesList: string[] = [];
export let sources: Adapter[] = [];
const octokitClient = new Octokit();

export function getAdapter(adapterID: string) {
  const source = sources.find((source) => source.config.platform === adapterID);
  if (!source) throw new Error("source adapter cannot be found");
  return source;
}

export async function updateAdapter(adapterID: string, path: string) {
  const adapter = getAdapter(adapterID);
  if (!adapter.config.githubRepo) return;
  const splitRepo = adapter.config.githubRepo.split("/");
  if (splitRepo.length !== 2) throw new Error("not a valid github repo");
  const [repo, owner] = splitRepo;

  const latestRelease = await octokitClient.rest.repos.getLatestRelease({
    repo,
    owner,
  });

  const asset = latestRelease.data.assets.find(
    (asset) => asset.label === `${adapter.config.platform}.js`,
  );
  if (!asset) {
    throw new Error(`asset named ${adapter.config.platform}.js not found.`);
  }

  // yes I know this isn't performant but it works
  const downloadRes = await (await fetch(asset.url)).arrayBuffer();
  await fs.writeFile(path, Buffer.from(downloadRes));
}

export async function reimportSources() {
  sources = await Promise.all(
    sourcesList.map(async (source) => await import(source)),
  );
}

/**
 * Adds a source js file to the list of sources
 * @param pathToSource the full path to the .js file
 */
export async function addAdapter(pathToSource: string) {
  sourcesList.push(pathToSource);
  await reimportSources();
}

export async function removeAdapter(pathToSource: string) {
  const newSourcesList = sourcesList.filter((source) =>
    source !== pathToSource
  );

  if (newSourcesList === sourcesList) {
    throw new Error("source not found or was not able to be removed");
  }

  sourcesList = newSourcesList;
  await reimportSources();
}

export async function setSourcesList(list: string[]) {
  sourcesList = list;
  await reimportSources();
}
