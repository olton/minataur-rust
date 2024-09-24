globalThis.UNKNOWN = `<span class="text-muted">UNKNOWN</span>`
globalThis.SLOT_DURATION = 180000
globalThis.EPOCH_DURATION = 1285200000
globalThis.GENESIS_START = "2024-06-04T16:00:00.000000-08:00"
// globalThis.GENESIS_START = "2021-03-17 02:00:00.000000+02:00"
globalThis.HARD_FORK_START = "2024-06-05 03:00:00.000000+02:00"
globalThis.TRANS_HASH_MARKER = "5J"
globalThis.BLOCK_HASH_MARKER = "3N"
globalThis.ACCOUNT_HASH_MARKER = "B62q"

var loader

const showLoader = () => {
    setTimeout(() => {
        loader =  Metro.activity.open({
            type: 'simple',
            style: 'color',
            overlayColor: '#fff',
            overlayAlpha: .2
        });
    })
}

const closeLoader = () => {
    if (loader) Metro.activity.close(loader)
}

// console.info(`%c
//
// ███╗   ███╗██╗███╗   ██╗ █████╗ ████████╗ █████╗ ██╗   ██╗██████╗
// ████╗ ████║██║████╗  ██║██╔══██╗╚══██╔══╝██╔══██╗██║   ██║██╔══██╗
// ██╔████╔██║██║██╔██╗ ██║███████║   ██║   ███████║██║   ██║██████╔╝
// ██║╚██╔╝██║██║██║╚██╗██║██╔══██║   ██║   ██╔══██║██║   ██║██╔══██╗
// ██║ ╚═╝ ██║██║██║ ╚████║██║  ██║   ██║   ██║  ██║╚██████╔╝██║  ██║
// ╚═╝     ╚═╝╚═╝╚═╝  ╚═══╝╚═╝  ╚═╝   ╚═╝   ╚═╝  ╚═╝ ╚═════╝ ╚═╝  ╚═╝
//
//         `, `color: #8f00ff;`)
//
