;
const lineChartOptions = {
    chart: {
        type: 'line',
        toolbar: {
            show: false
        },
        animations: {
            enabled: false,
            speed: 300
        },
        height: 150,
        parentHeightOffset: 0,
    },
    xaxis: {
        labels: {
            show: false
        }
    },
    noData: {
        text: 'Loading...'
    },
    tooltip: {
        x: {
            formatter: (val, obj) => {
                return `Block: ${val}`
            }
        },
        y: {
            formatter: (val, obj) => {
                return `${val}`
            }
        },
    },
    stroke: {
        show: true,
        curve: 'straight',
        lineCap: 'butt',
        colors: undefined,
        width: 2,
        dashArray: 0,
    }
}

const updateCharts = data => {
    drawChartTPB(data)
    drawChartUserTPB(data)
    drawChartIntTPB(data)
    drawChartZkappTPB(data)
    drawChartSPB(data)
    drawChartFPB(data)
    drawChartPAR(data)
    drawChartAvgTxAmount(data)
}

let chartTPB,
    chartZkappTPB,
    chartIntTPB,
    chartUserTPB,
    chartSPB,
    chartFPB,
    chartPAR,
    chartAvgAmount,
    chartPrice

const drawChartTPB = data => {
    const dd = []
    for(let r of data.reverse()) {
        dd.push({
            y: +r.user_trans_count + +r.internal_trans_count + +r.zkapp_trans_count,
            x: +r.height
        })
    }
    if (!chartTPB) {
        chartTPB = new ApexCharts(document.querySelector("#chart-tpb"), merge({}, lineChartOptions, {
            series: [{
                name: 'Trans',
                data: dd
            }],
            yaxis: {
                tickAmount: 5,
            },
        }))
        chartTPB.render();
    } else {
        chartTPB.updateSeries([{
            name: 'Trans',
            data: dd
        }])
    }
}

const drawChartZkappTPB = data => {
    const dd = []
    for(let r of data) {
        dd.push({
            y: +r.zkapp_trans_count,
            x: +r.height
        })
    }
    if (!chartZkappTPB) {
        chartZkappTPB = new ApexCharts(document.querySelector("#chart-zkapp-tpb"), merge({}, lineChartOptions, {
            series: [{
                name: 'ZkApp Tx',
                data: dd
            }],
            colors: ['#af9528'],
            yaxis: {
                tickAmount: 5,
            },
        }))
        chartZkappTPB.render();
    } else {
        chartZkappTPB.updateSeries([{
            name: 'ZkApp Tx',
            data: dd
        }])
    }
}

const drawChartIntTPB = data => {
    const dd = []
    for(let r of data) {
        dd.push({
            y: +r.internal_trans_count,
            x: +r.height
        })
    }
    if (!chartIntTPB) {
        chartIntTPB = new ApexCharts(document.querySelector("#chart-int-tpb"), merge({}, lineChartOptions, {
            series: [{
                name: 'Int Tx',
                data: dd
            }],
            colors: ['#ff1841'],
            yaxis: {
                min: 0,
                tickAmount: 5,
            },
        }))
        chartIntTPB.render()
    } else {
        chartIntTPB.updateSeries([{
            name: 'Int Tx',
            data: dd
        }])
    }
}

const drawChartUserTPB = data => {
    const dd = []
    for(let r of data) {
        dd.push({
            y: +r.user_trans_count,
            x: +r.height
        })
    }
    if (!chartUserTPB) {
        chartUserTPB = new ApexCharts(document.querySelector("#chart-user-tpb"), merge({}, lineChartOptions, {
            series: [{
                name: 'User Tx',
                data: dd
            }],
            colors: ['#4ba228'],
            yaxis: {
                tickAmount: 5,
            },
        }))
        chartUserTPB.render();
    } else {
        chartUserTPB.updateSeries([{
            name: 'User Tx',
            data: dd
        }])
    }
}

const drawChartSPB = data => {
    const dd = []
    for(let r of data) {
        dd.push({
            y: +r.block_slots,
            x: +r.height
        })
    }
    if (!chartSPB) {
        chartSPB = new ApexCharts(document.querySelector("#chart-spb"), merge({}, lineChartOptions, {
            series: [{
                name: 'Slots',
                data: dd
            }],
            colors: ['#ff7f50'],
            yaxis: {
                min: 0,
                tickAmount: 5
            },
        }))
        chartSPB.render()
    } else {
        chartSPB.updateSeries([{
            name: 'Slots',
            data: dd
        }])
    }
}

const drawChartFPB = data => {
    const dd = []
    for(let r of data) {
        dd.push({
            y: +r.trans_fee/10**9,
            x: +r.height
        })
    }
    if (!chartFPB) {
        chartFPB = new ApexCharts(document.querySelector("#chart-fpb"), merge({}, lineChartOptions, {
            series: [{
                name: 'Fee',
                data: dd
            }],
            yaxis: {
                decimalsInFloat: 4,
                tickAmount: 5,
            },
            colors: ['#9932cc'],
        }))
        chartFPB.render()
    } else {
        chartFPB.updateSeries([{
            name: 'Fee',
            data: dd
        }])
    }
}

const drawChartPAR = data => {
    const dd = []
    for(let r of data) {
        dd.push({
            y: +r.block_participants,
            x: +r.height
        })
    }
    if (!chartPAR) {
        chartPAR = new ApexCharts(document.querySelector("#chart-par"), merge({}, lineChartOptions, {
            series: [{
                name: 'Par',
                data: dd
            }],
            yaxis: {
                min: 0,
                tickAmount: 5,
            },
            colors: ['#af9528']
        }))
        chartPAR.render()
    } else {
        chartPAR.updateSeries([{
            name: 'Par',
            data: dd
        }])
    }
}

const drawChartAvgTxAmount = data => {
    const dd = []
    for(let r of data) {
        dd.push({
            y: +(+r.trans_amount/10**9).toFixed(0),
            x: +r.height
        })
    }
    if (!chartAvgAmount) {
        chartAvgAmount = new ApexCharts(document.querySelector("#chart-avg-tx-amount"), merge({}, lineChartOptions, {
            series: [{
                name: 'Avg Amount Volume',
                data: dd
            }],
            yaxis: {
                min: 0,
                tickAmount: 5
            },
            colors: ['#1a12ec']
        }))
        chartAvgAmount.render()
    } else {
        chartAvgAmount.updateSeries([{
            name: 'Avg Amount Volume',
            data: dd
        }])
    }
}

const drawPriceChart = data => {
    const dd = []
    for(let r of data.reverse()) {
        dd.push({
            y: +r.value,
            x: datetime(r.timestamp).time()
        })
    }
    console.log(dd)
    if (!chartPrice) {
        chartPrice = new ApexCharts($("#price-chart-sparkle")[0], {
            chart: {
                type: 'area',
                height: 20,
                sparkline: {
                    enabled: true
                },
                animations: {
                    enabled: false,
                },
                toolbar: {
                    show: false
                },
            },
            series: [{
                name: "Price",
                data: dd
            }],
            stroke: {
                curve: 'straight',
                width: 1,
            },
            fill: {
                opacity: 0.3,
            },
            colors: ['#50a8ff'],
        })
        chartPrice.render()
    } else {
        chartPrice.updateSeries([{
            name: "Price",
            data: dd
        }])
    }
}