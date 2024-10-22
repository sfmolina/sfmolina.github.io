//-------------------------------------------------------------------//
//  AUTHOR:    @sfmolina                                            //
//  Version:   v1                                                  //
//  Modified:  23oc24                                             //
//---------------------------------------------------------------//



use yew::prelude::*;
use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;



#[wasm_bindgen(inline_js = "
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
")]
extern "C" {
    fn init_chart(data: &JsValue);
}


#[function_component(EchartDemo)]
pub fn echart_demo() -> Html {

    let sales: Vec<i32> = vec![5, 20, 36, 10, 10, 50];

    use_effect(move || {
        let js_data = to_value(&sales).unwrap();
        init_chart(&js_data);
        || ()
    });

    html! {
        <div id="main" style="width: 900px;height:400px;"></div>
    }
}
