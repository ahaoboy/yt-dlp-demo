type Version = {
  ytdlp: string,
  ffmpeg: string
}

export async function getVersion(): Promise<Version | undefined> {
  return (await (await fetch("/get_version")).json()) as Version
}