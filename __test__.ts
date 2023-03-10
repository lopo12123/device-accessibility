import {helloworld, Controller, Observer} from "./index.js"
import type {ExtraKey, KeyEv, KeyCombination} from "./index.js"

// region 可用性测试
helloworld()  // => Just a classic hello-world.
// endregion

// region 控制
// 0. 实例化控制类
const ct = new Controller()

// 1. 控制点击按键
ct.keyClick({key: 'KeyA'})

// 2. 控制点击组合键
ct.keyClick({key: 'KeyA', extra: {ctrl: true}})
ct.keyClick({key: 'KeyA', extra: {ctrl: true, shift: true}})
ct.keyClick({key: 'KeyA', extra: {ctrl: true, shift: true, alt: true}})
ct.keyClick({key: 'KeyA', extra: {ctrl: true, shift: true, alt: true, meta: true}})

// 3. 控制按键 (按下 -- 1s -- 释放)
ct.keyDown('KeyA')
setTimeout(() => ct.keyUp('KeyA'), 1_000)

// 4. 控制输入字符串
ct.keyType('hello world')

// 5. 控制鼠标移动到 (100px, 100px) (以主屏幕左上角为原点)
ct.mouseMove({x: 100, y: 100})

// 6. 控制鼠标从当前位置向右向下各移动 100px
ct.mouseMove({x: 100, y: 100}, true)

// 7. 控制鼠标左键单击
ct.mouseClick('Left')

// 8. 控制鼠标中键单击
ct.mouseClick('Middle')

// 9. 控制鼠标右键单击
ct.mouseClick('Right')

// 10. 控制鼠标 (按下 -- 1s -- 释放)
ct.mouseDown('Left')
setTimeout(() => ct.mouseUp('Left'), 1_000)

// 11. 控制鼠标滚动 (具体滚动距离在不同平台有不同表现)
ct.mouseScroll(-1)  // 向上一个单位
ct.mouseScroll(1)  // 向下一个单位
ct.mouseScroll(-1, true)  // 向左一个单位
ct.mouseScroll(1, true)  // 向右一个单位

// 12. 获取当前鼠标位置 (以主屏幕左上角为原点)
const position = ct.mouseLocation()
// endregion

// region 监听
// 0. 实例化监听器
const ob = new Observer()

// 1. 判断按键可用性
ob.checkKey('KeyA')  // => true

// 2. 监听按键 (默认为释放时触发, 指定 `down` 为 `true` 在按下时触发)
ob.onKey({key: 'KeyA'}, () => {
    console.log('按键A被释放')
})
ob.onKey({key: 'KeyA', down: true}, () => {
    console.log('按键A被按下')
})

// 3. 监听组合按键
// - 先按下目标按键再按下辅助按键不触发回调
// - `meta`: windows -- `win`; macos -- `command`; linux -- `super`
ob.onKey({key: 'KeyA', extra: {ctrl: true}, down: true}, () => {
    console.log('按键 ctrl + A 被按下')
})
ob.onKey({key: 'KeyA', extra: {ctrl: true, shift: true}, down: true}, () => {
    console.log('按键 ctrl + shift + A 被按下')
})
ob.onKey({key: 'KeyA', extra: {ctrl: true, shift: true, alt: true}, down: true}, () => {
    console.log('按键 ctrl + shift + alt + A 被按下')
})
ob.onKey({key: 'KeyA', extra: {ctrl: true, shift: true, alt: true, meta: true}, down: true}, () => {
    console.log('按键 ctrl + shift + alt + meta + A 被按下')
})

// 4. 取消监听按键
ob.offKey({key: 'KeyA'})
ob.offKey({key: 'KeyA', down: true})

// 5. 取消监听组合键
ob.offKey({key: 'KeyA', extra: {ctrl: true}})
ob.offKey({key: 'KeyA', extra: {ctrl: true, shift: true}})
ob.offKey({key: 'KeyA', extra: {ctrl: true, shift: true, alt: true}})
ob.offKey({key: 'KeyA', extra: {ctrl: true, shift: true, alt: true, meta: true}})

// 6. 监听所有按键事件
ob.onKeyAll((err, key_ev) => {
    console.log(`按键 ${key_ev.key} 被 ${key_ev.down ? '按下' : '释放'}`)
})

// 7. 取消监听所有按键事件
ob.offKeyAll()

// 8. 主动触发一次监听回调
ob.touch({key: 'KeyA'})
ob.touch({key: 'KeyA', down: true})
ob.touch({key: 'KeyA', extra: {ctrl: true}})
ob.touch({key: 'KeyA', extra: {ctrl: true}, down: true})

// 9. 查看已注册的按键事件
ob.registeredKeys  // KeyEv[]

// 10. 结束监听 (必须手动调用以释放引用, 否则当前线程会始终存活且阻止所有处于注册状态的事件回调函数触发gc)
ob.dispose()
// endregion