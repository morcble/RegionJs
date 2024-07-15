(function flexible (window, document) {
    document.addEventListener('DOMContentLoaded', setCharacterSize);
    var dpr = window.devicePixelRatio || 1 //设备像素比
    function setCharacterSize(){
        var userAgentInfo = navigator.userAgent;
        // var Agents = ["Android", "iPhone","SymbianOS", "Windows Phone","iPad", "iPod"];
        var Agents = ["Android", "iPhone","SymbianOS", "Windows Phone","iPod"];
        var isPc = true;
        for (var v = 0; v < Agents.length; v++) {
            if (userAgentInfo.indexOf(Agents[v]) > 0) {
                isPc = false;
                break;
            }
        }
        // 获取浏览器可视区域宽高（兼容性比较好，不包括工具栏和滚动条）
        var browserWidth = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
        var browserHeight = window.innerHeight || document.documentElement.clientHeight || document.body.clientHeight;

        /*var scaleParaPerLine = 8;//手机端行系数
        if(isPc){
            scaleParaPerLine = 128;//PC行系数
        }

        var fontSize = browserWidth/scaleParaPerLine;
        if(fontSize<8){
            fontSize = 8;
            scaleParaPerLine = browserWidth/fontSize;
        }*/
        
        var scaleParaPerLine = 128;//行系数

        var fontSize = browserWidth/scaleParaPerLine;

        document.documentElement.style.fontSize = fontSize + 'px';
        document.body.style.fontSize = '1rem';
    }

}(window, document))

