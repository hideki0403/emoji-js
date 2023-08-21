import { generate } from '..'

describe('basic test', () => {
    // バイナリが返される
    it('return binary', () => {
        expect(generate('emoji')).toBeInstanceOf(Buffer)
    })

    // 間違った色を指定するとエラーになる
    it('throw error when wrong color', () => {
        expect(() => generate('emoji', { color: 'wrong color' })).toThrowError()
    })

    // 間違ったtextAlignを指定するとエラーになる
    it('throw error when wrong textAlign', () => {
        expect(() => generate('emoji', { textAlign: 'wrong textAlign' as any })).toThrowError()
    })

    // 間違ったフォーマットを指定するとエラーになる
    it('throw error when wrong format', () => {
        expect(() => generate('emoji', { format: 'wrong format' as any })).toThrowError()
    })
})

// プラットフォームによってレンダリングされる画像に差があり、問題がない場合でもテストが失敗することがあるためにデフォルトでは実行しない
describe.skip('binary test', () => {
    // 絵文字を生成できる
    it('generate emoji', () => {
        expect(generate('emoji')).toMatchImageSnapshot()
    })

    // 指定したサイズで生成できる
    it('generate emoji with size', () => {
        expect(generate('emoji', { width: 512, height: 512 })).toMatchImageSnapshot()
    })

    // 指定した色で生成できる
    it('generate emoji with color', () => {
        expect(generate('emoji', { color: '#A3BE8C' })).toMatchImageSnapshot()
    })

    // 指定した背景色で生成できる
    it('generate emoji with background color', () => {
        expect(generate('emoji', { backgroundColor: '#A3BE8C' })).toMatchImageSnapshot()
    })

    // textSizeFixedを指定できる
    it('generate emoji with textSizeFixed', () => {
        expect(generate('emoji\ngen', { textSizeFixed: true })).toMatchImageSnapshot()
    })

    // disableStretchを指定できる
    it('generate emoji with disableStretch', () => {
        expect(generate('emoji\ngen', { disableStretch: true })).toMatchImageSnapshot()
    })

    // textAlignを指定できる
    it('generate emoji with textAlign', () => {
        expect(generate('emo\nji', { textAlign: 'left', textSizeFixed: true })).toMatchImageSnapshot()
    })

    // flexibleWidthを指定できる
    it('generate emoji with flexibleWidth', () => {
        expect(generate('emoji generator', { flexibleWidth: true })).toMatchImageSnapshot()
    })

    // typefaceFileを指定できる
    it('generate emoji with typefaceFile', () => {
        expect(generate('絵文字', { typefaceFile: 'test/assets/NotoSansJP-Regular.ttf' })).toMatchImageSnapshot()
    })
})

// 特定のプラットフォームでは実行できないテスト類
describe.skip('advanced test', () => {
    // typefaceNameを指定できる (フォントがインストールされていない環境では失敗する)
    it('generate emoji with typefaceName', () => {
        expect(generate('emoji', { typefaceName: 'Arial' })).toMatchImageSnapshot()
    })
    
    // フォーマットを指定できる (テストフレームワークがjpegに非対応のため、2回目以降のテストに失敗する)
    it('generate emoji with format', () => {
        expect(generate('emoji', { format: 'jpeg', color: '#FFFFFF' })).toMatchImageSnapshot()
    })
})