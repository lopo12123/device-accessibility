import { Controller, KeyCombination } from "../index.js"

function key_click() {
    const simulator = new Controller()
    const simKeys: KeyCombination = {
        key: 'KeyA', extra: { ctrl: true }
    }

    setTimeout(() => {
        simulator.keyClick(simKeys)
    }, 2000)
}

// key_click()

function mouse_click() {
    const simulator = new Controller()

    setTimeout(() => {
        simulator.mouseClick('Right')
    }, 2000)
}

mouse_click()

