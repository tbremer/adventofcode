import { readFile } from 'fs/promises';

export async function read(path: string) {
  return readFile(path);
}

export async function readToString(path: string, stringType: BufferEncoding = 'utf-8') {
  const buffer = await read(path);

  return buffer.toString(stringType);
}
