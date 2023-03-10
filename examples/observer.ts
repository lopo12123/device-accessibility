import { KeyCombination, Observer } from "../index.js"

function key_listen_spec() {
    const listener = new Observer()
    const targetKeys: KeyCombination = {
        key: 'KeyA', extra: { ctrl: true }
    }

    listener.onKey(targetKeys, (err) => {
        if(err) {
            console.log('error!', err)
        } else {
            console.log('key A clicked (release)')
        }
    })

    setTimeout(() => {
        listener.dispose()
    }, 5_000)
}

key_listen_spec()
