# @larscom/image-data

[![npm-version](https://img.shields.io/npm/v/@larscom/image-data.svg?label=npm)](https://www.npmjs.com/package/@larscom/image-data)
![npm](https://img.shields.io/npm/dw/@larscom/image-data)
[![license](https://img.shields.io/npm/l/@larscom/image-data.svg)](https://github.com/larscom/image-data/blob/main/LICENSE)

Easily construct an `Image` object from a URL or ArrayBuffer. An `Image` object contains the width and height of the image and an `ImageData` object. It uses `WASM` to cunstruct such an object.

## Installation

```bash
npm install @larscom/image-data
```

## Usage

```ts
import { load_from_url } from '@larscom/image-data'

const imageUrl = 'https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png'
const image = await load_from_url(imageUrl)

const canvas = document.getElementById('canvas')

canvas.width = image.width
canvas.height = image.height

const ctx = canvas.getContext('2d')

ctx.putImageData(image.data, 0, 0)
```
