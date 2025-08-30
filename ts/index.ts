import init, { get_image } from './wasm/image_data.js'

const _init = init()

export interface Image {
  width: number
  height: number
  data: ImageData
  // RGBA
  channels: Uint8Array
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
  const data = new ImageData(new Uint8ClampedArray(channels.buffer) as Uint8ClampedArray<ArrayBuffer>, width, height)

  return {
    width,
    height,
    data,
    channels
  }
}

/**
 * Convert each individual pixel to a HEX color code.
 *
 * @param channels The RGBA channels
 * @returns an array of hex color codes (e.g: #F54927)
 */
export function convert_to_hex_colors(channels: Uint8Array): string[] {
  const colors = []
  for (let i = 0; i < channels.length; i += 4) {
    const r = channels[i]!
    const g = channels[i + 1]!
    const b = channels[i + 2]!
    const a = channels[i + 3]!
    colors.push(rgba_to_hex(r, g, b, a))
  }
  return colors
}

export interface Pixel {
  r: number
  g: number
  b: number
  a: number
}

/**
 * Decode raw channel data into RGBA
 * 
 * @param channels The RGBA channels
 * @returns an array of pixels
 */
export function decode_to_rgba(channels: Uint8Array): Pixel[] {
  const pixels = []
  for (let i = 0; i < channels.length; i += 4) {
    const r = channels[i]!
    const g = channels[i + 1]!
    const b = channels[i + 2]!
    const a = channels[i + 3]!
    pixels.push({ r, g, b, a })
  }
  return pixels
}

function rgba_to_hex(r: number, g: number, b: number, a: number): string {
  return `#${r.toString(16).padStart(2, '0')}${g.toString(16).padStart(2, '0')}${b.toString(16).padStart(2, '0')}${a
    .toString(16)
    .padStart(2, '0')}`.toUpperCase()
}
