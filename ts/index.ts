import init, { get_image } from './pkg/file_inspector.js'

const _init = init()

export async function load_from_arraybuffer(buffer: ArrayBuffer): Promise<ImageData> {
  await _init

  const image = get_image(new Uint8Array(buffer))
  const data = new Uint8ClampedArray(image.data().buffer) as Uint8ClampedArray<ArrayBuffer>

  return new ImageData(data, image.width(), image.height())
}
