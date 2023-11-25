type GistResp = {
  url: string;
  id: string;
  files: Record<string, GistFile>;
};

type GistFile = {
  filename: string;
  raw_url: string;
  size: number;
  truncated: boolean;
  content: string;
};

// I'm borrowing some of this from the modmark-org/modmark repo
export default async function readGist(
  id: string,
): Promise<string> {
  let api_result: GistResp;
  try {
    const res = await fetch("https://api.github.com/gists/" + id);
    if (res.status !== 200) {
      return `-- Error fetching Gist with id ${id}: status code ${res.status}`;
    }
    const content = await res.text();
    api_result = JSON.parse(content) as GistResp;
  } catch (e) {
    return `-- Error loading Gist: ${e}`;
  }

  const entries = Object.entries(api_result.files);

  if (entries.length === 0) {
    return "-- No files found in Gist";
  }

  if (entries.length === 1) {
    const [_, file] = entries[0];
    return file.content;
  } else {
    return "-- Gist contains multiple files! I can't choose which one to use.";
  }
}
