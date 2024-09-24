globalThis.webSocket = null

const isOpen = (ws) => ws && ws.readyState === ws.OPEN

const connect = () => {
    const {server_address, server_port, server_secure = false} = globalThis.config
    const ws = new WebSocket(`${server_secure ? 'wss' : 'ws'}://${server_address}:${server_port}/ws/`)

    globalThis.webSocket = ws

    ws.onmessage = event => {
        try {
            const content = JSON.parse(event.data)
            if (typeof globalThis["wsController"] === 'function') {
                globalThis["wsController"].apply(null, [ws, content])
            }
        } catch (e) {
            log(e.message)
            log(event.data)
            log(e.stack)
        }
    }

    ws.onerror = error => {
        Metro.toast.create(`Connection to server lost!`, {clsToast: "alert"})
        error('Socket encountered error: ', error.message, 'Closing socket');
        ws.close();
    }

    ws.onclose = event => {
        $(".live").addClass("alert")
        log('Socket is closed. Reconnect will be attempted in 1 second.', event.reason);
        setTimeout(connect, 1000)
    }

    ws.onopen = event => {
        $(".live").removeClass("alert")
        log('Connected to Minataur, wait for welcome message!');
        request("welcome", "Hi", ws)
    }
}

const request = (channel, data, ws = globalThis.webSocket) => {
    if (isOpen(ws)) {
        ws.send(JSON.stringify({channel, data}));
    }
}

connect()
