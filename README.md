# emoji-js
[![npm version](https://img.shields.io/npm/v/@hideki0403%2Femoji.js)](https://www.npmjs.com/package/@hideki0403/emoji.js)
[![CI](https://github.com/hideki0403/emoji-js/actions/workflows/CI.yml/badge.svg)](https://github.com/hideki0403/emoji-js/actions/workflows/CI.yml)  

Unofficial reimplementation of [libemoji.js](https://github.com/nzws/libemoji.js) and [libemoji](https://github.com/emoji-gen/libemoji) in Rust.  

## Usage
```sh
npm i @hideki0403/emoji.js
# or
yarn add @hideki0403/emoji.js
# or
pnpm i @hideki0403/emoji.js
```

```js
import { generate } from '@hideki0403/emoji.js'
import fs from 'fs'

const buffer = generate('絵\n文字', {
    width: 128,
    height: 128,
    // ... See below for other options
})
fs.writeFileSync('emoji.png', buffer)
```

### Options

#### `width`  
  
Default: `128`  
  
The width of the generated image.  
  
#### `height`
  
Default: `128`
  
The height of the generated image.
  
#### `flexible_width`
  
Default: `false`
  
If `true`, the width of the generated image will be flexible, allowing you to create wide emojis. The `width` option will be ignored.  
  
#### `color`
  
Default: `#000000`  

The color of the emoji.  
  
#### `background_color`
  
Default: `#00000000`
  
The background color of the emoji.  
  
#### `text_align`
  
Default: `center`  
Enum: `left`, `center`, `right`
  
The alignment of the text.  
  
#### `text_size_fixed`  
  
Default: `false`  
  
If `true`, the text size will be fixed.  
  
#### `disable_stretch`  
  
Default: `false`  
  
If `true`, the text will not be stretched.  
  
#### `disable_outline`
  
Default: `false`  
  
If `true`, the outline will not be drawn.  
  
#### `outline_width`
  
Default: `8`  
  
The width of the outline.  
  
#### `outline_color`  
  
Default: `#ffffff`  
  
The color of the outline.  
  
#### `typeface_file`
  
Default: `undefined`  
  
The path to the font file.  
  
#### `typeface_name`
  
Default: `undefined`  
  
The typeface name of the font.  
  
#### `format`
  
Default: `png`  
Enum: `png`, `jpeg`
  
The format of the generated image.  
  
#### `quality`
  
Default: `100`  
  
The quality of the generated image.  

## Other
  
### Why reimplementation? (in Japanese) / 再実装した理由について
これまで他のプロジェクトで絵文字生成に[libemoji.js](https://github.com/nzws/libemoji.js)を使用させて頂いていたのですが、ラッパー先の[libemoji](https://github.com/emoji-gen/libemoji)はLinuxのみ対応であることや、libemojiがSkiaに依存しており、Skiaのビルドから行わなければならない (ビルドには1時間以上掛かる) ということが判明しました。  
上記に加え、libemoji.jsをWindows/Macに対応させるにはlibemoji自体から対応しなければならないことや、CI上で別々のプラットフォーム向けにSkiaをビルドするには非常に手間が掛かること等を踏まえ、Skiaがプレビルドされており、なおかつSkiaへのバインディングが存在するRustでlibemojiおよびlibemoji.jsを再実装し、Windows/Macに対応することにしました。  

### Special Thanks
emoji.jsを開発するにあたり、libemojiおよびlibemoji.jsの様々な箇所を参考にさせて頂きました。  
この場を借りて、libemojiおよびlibemoji.jsの開発者の方々に感謝申し上げます。  
