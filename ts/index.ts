import init, { get_image } from './wasm/image_data.js'

const _init = init()

export interface Image {
  width: number
  height: number
  data: ImageData
  channels: Uint8Array<ArrayBuffer>
}

export async function load_from_url(url: RequestInfo): Promise<Image> {
  await _init

  const response = await fetch(url)
  const buffer = await response.arrayBuffer()

  return load_from_arraybuffer(buffer)
}

export async function load_from_arraybuffer(buffer: ArrayBuffer): Promise<Image> {
  await _init

  const image = get_image(new Uint8Array(buffer))
  const width = image.width()
  const height = image.height()
  const channels = image.data()
  const data = new ImageData(
    new Uint8ClampedArray(channels.buffer) as Uint8ClampedArray<ArrayBuffer>,
    width,
    height
  )

  return {
    width,
    height,
    data,
    channels
  }
}
