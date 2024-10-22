
export function init_chart(data) {
    var myChart = echarts.init(document.getElementById('main'));
    var option = {
        title: {
            text: 'ECharts Getting Started Example'
        },
        tooltip: {},
        legend: {
            data: ['sales']
        },
        xAxis: {
            data: ['Shirts', 'Cardigans', 'Chiffons', 'Pants', 'Heels', 'Socks']
        },
        yAxis: {},
        series: [
            {
                name: 'sales',
                type: 'bar',
                data: data
            }
        ]
    };
    myChart.setOption(option);
}
