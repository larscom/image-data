import { useEffect } from 'react'
import { load_from_url } from '@larscom/image-data'
import './App.css'

function App() {
  useEffect(() => {
    load_from_url('https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png').then(
      (image) => {
        const canvas = document.getElementById('canvas') as HTMLCanvasElement
        canvas.width = image.width
        canvas.height = image.height

        const ctx = canvas.getContext('2d')!

        ctx.putImageData(image.data, 0, 0)
      }
    )
  }, [])

  return <canvas id="canvas"></canvas>
}

export default App
