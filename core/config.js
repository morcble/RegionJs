		var test = document.getElementById("configjs");
	    var src = test.getAttribute("src");
	    var theRequest = new Object();
	    if (src.indexOf("?") != -1) {
	        var str = src.substr(src.indexOf('?') + 1);
	        var strs = str.split("&");
	        for (var i = 0; i < strs.length; i++) {
	            theRequest[strs[i].split("=")[0]] = unescape(strs[i].split("=")[1]);
	        }
	    }


		var Config = {
				skin : "skin1",
				locale:"cn",//cn en
				REGION_DEBUG_MODE : false,
				lazyLoadImg:true,
				releaseVersion : theRequest["v"],
				frontendPath :  "/",
				//backendPath  : window.location.protocol+"//"+document.domain+":8888/region",
				backendPath: window.location.protocol + "//" +window.location.host + "/region",
				encrypt:false,
		    	resizeAble:false,
		    	rsRepo:"http://cdn.rcsdn.cn/rsrepo/",
		    	//rsRepo:"http://rs.innovationpark.regionsoft.cn:8888/tutorial/basic/lesson120/repo/",
		    	requestInterceptor:null,//发送http请求的拦截器
		    	responseInterceptor:null,//接收http请求的拦截器
		    	netErrorInterceptor:null,//网络错误的拦截器
		    	fileVendor:"local"//全局文件存储方式
		}
//		Config.requestInterceptor = function(reqData){
//			if(reqData==null)reqData={};
//			var reqWrapper = {};
//			reqWrapper.data = JSON.stringify(reqData);
//			reqWrapper.requestId= getUUID2().replaceAll("-","");
//			
//			var requestObj = {};
//			requestObj.requestStr = JSON.stringify(reqWrapper);
//			return requestObj;
//		}

		//获取国际化
		var locale = null;
		var arr, reg = new RegExp("(^| )locale=([^;]*)(;|$)");
		if (arr = document.cookie.match(reg))
			locale = unescape(arr[2]);
		if(locale!=null){
			Config.locale = locale;
		}
		var targetLocalPath=Config.locale==""?"":"_"+Config.locale;
		//获取国际化结束

		var ResFilePrefix = Config.frontendPath;	
		//var ResFilePrefix = window.location.protocol+"//cdn.regionsoft.cn/shared/";
		var jsDependencies = [
		               ResFilePrefix+"libs/jquery/jquery-1.12.4.min.js",



		               ResFilePrefix+"libs/regionjs/4/regionjs-4.0.0.full.release.js",
		               ResFilePrefix+"libs/regionjs/4/regionjs.ext-4.0.0.full.release.js",


		               Config.frontendPath + "messages"+targetLocalPath+"/global_msg.js",
		               Config.frontendPath+"core/datals.js",
		               ResFilePrefix+"libs/moment/moment.min.js",
		               ResFilePrefix+"libs/jquery/jquery.lazyload.1.9.3.js",

		               ResFilePrefix+"libs/ckeditor/ckeditor.js",
		               ResFilePrefix+"libs/barcode/3.11/JsBarcode.all.min.js",
		               ResFilePrefix+"libs/qrcode/qrcode.min.js",
		               ResFilePrefix+"libs/echarts/echarts-5.3.1.min.js",
		               ResFilePrefix+"libs/echarts/echarts-gl.min.js",
		               
		               ResFilePrefix+"libs/regionext/graph/graph-1.0.js",

		               //云服务相关
		               ResFilePrefix+"libs/qiniu.min.js",
		               ResFilePrefix+"libs/spark-md5.js",
		               ResFilePrefix+"libs/aes.js",
		               ResFilePrefix+"libs/pako.min.js",

					   Config.frontendPath+"js/rem.js",
					   Config.frontendPath+"js/theme.js",//主题设置
		             ];

		var cssDependencies = [
						ResFilePrefix+"libs/fontawesome-6.2.0/assets/fontawesome/css/all_pro.css",
						ResFilePrefix+"libs/bootstrap/bootstrap.min.css",
						Config.frontendPath+"css/"+Config.skin+"/cssSkin.css",

						ResFilePrefix+"libs/animate.css/4.0/animate.min.css",
		              ];


for(var i = 0 ; i <cssDependencies.length ; i++){
	document.write("<link href=\""+cssDependencies[i]+"?v="+Config.releaseVersion+"\" rel=\"stylesheet\"></link>");
}

for(var i = 0 ; i <jsDependencies.length ; i++){
	document.write("<script src=\""+jsDependencies[i]+"?v="+Config.releaseVersion+"\"><\/script>");
}


//validtions
var emptyReg = /^(\s*\S+\s*)+$/;///^\s+$/;
var emptyArray = /^\[(\s*\S+\s*)+\]$/;///^\s+$/;
var numberReg = /^[0-9]*$/;///^\s+$/;
var decimal = /^([1-9]+[\d]*(.[0-9]{1,2})?)$/
var numbergtReg = /^(\+?[1-9]\d*)?$/;///^\s+$/;
var currencyReg =/(^[1-9]([0-9]+)?(\.[0-9]{1,2})?$)|(^(0){1}$)|(^[0-9]\.[0-9]([0-9])?$)|^$/;
var mobileReg=/^1(?:3\d|4[4-9]|5[0-35-9]|6[67]|7[013-8]|8\d|9\d)\d{8}$/
var nullormobileReg=/^1\d{10}|((0\d{2,3}-\d{7,8})|(1[3567849]\d{9}))|^$/;
var phoneReg=/^0\d{2,3}-?\d{7,8}$/;
var accountReg=/^[a-zA-z]\w{3,15}$/;
var emailReg= /^(\w-*\.*)+@(\w-?)+(\.\w{2,})+$/;
var timeReg = /^(([0-1][0-9])|(2[0-3])):[0-5][0-9]$/;
var englishReg = /[^a-zA-Z]/g;
var notzh = /^[-0-9a-zA-Z_]*$/;
var ipReg = /^(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])\.(\d{1,2}|1\d\d|2[0-4]\d|25[0-5])$/;
var urlReg = /^(https?|ftp|file):\/\/[-A-Za-z0-9+&@#/%?=~_|!:,.;]+[-A-Za-z0-9+&@#/%=~_|]$/;
var idNo=/^[1-9]\d{5}(18|19|20|(3\d))\d{2}((0[1-9])|(1[0-2]))(([0-2][1-9])|10|20|30|31)\d{3}[0-9Xx]$/;
//身份证验证
var idcardreg = /(^\d{15}$)|(^\d{18}$)|(^\d{17}(\d|X|x)$)/;
var pwdReg = /^(\w){6,20}$/;///^(?=.*[A-Z])(?=.*[0-9])(?!.*([~!bai@&%$^\(\)#_]).*\1.*\1)[A-Z0-9~!@&%$^\(\)#_]{8,16}$/;
var allNumReg = /^([+-]?(0|([1-9]\d*))(\.\d+)?$)?$/;
var positiveNumReg = /^(\d+(\.{0,4}\d+){0,4})?$/;
var telReg=/^((0\d{2,3}-\d{7,8})|(1[3567849]\d{9}))|(^$)$/;
//不以0开头
var numberReg0 = /^[1-9]\d*$/;
//金钱  小数点 前9位 后2位 0不保存
var nameReg = /^[a-zA-Z\u4e00-\u9fa5]+$/
var moneyNumReg = /^(?:0\.\d{0,1}[1-9]|(?!0)\d{1,9}(?:\.\d{0,1}[1-9])?)$/;
//金钱  小数点 前9位 后6位 0不保存
var moneyNumReg1 = /^(?:0\.\d{0,5}[1-9]|(?!0)\d{1,9}(?:\.\d{0,5}[1-9])?)$/;
//保留两位 不过滤0
var moneyNumReg0 = /^(([1-9]{1}\d*)|(0{1}))(\.\d{0,2})?$/;
var currencygtReg =/(^[+-][1-9]([0-9]+)?(\.[0-9]{1,2})?$)|(^(0){1}$)|(^[0-9]\.[0-9]([0-9])?$)|^$/;
var cellPhoneReg = /^1(3[0-9]|4[01456879]|5[0-35-9]|6[2567]|7[0-8]|8[0-9]|9[0-35-9])\d{8}$/;
var taxNoReg = /[0-9A-HJ-NPQRTUWXY]{2}\d{6}[0-9A-HJ-NPQRTUWXY]{10}/;
var bankAccountReg = /^([1-9]{1})(\d{14}|\d{18})$/;
// 统一社会信用代码 18位或15位
var socialCodeReg = /^([0-9A-HJ-NPQRTUWXY]{2}\d{6}[0-9A-HJ-NPQRTUWXY]{10}|[1-9]\d{14})$/;
// 0-100的整数
var numtypeoneReg = /^([0-9]{1,2}|100)$/;
// 0-100的保留两位小数
var numtypetwoReg = /(^(\d|[1-9]\d)(\.\d{1,2})?$)|(^100$)/;

// 0-100的保留两位小数
var numtypetenReg = /(^(10|[0-9])$)/;
// 0-100的保留两位小数
var numtypetwentyReg = /(^(20|[0-9]|[1][0-9])$)/;

//正数  最多保留2位
var numReg2 = /^(([1-9]{1}\d*)|(0{1}))(\.\d{1,2})?$|^$/;

//正数  最多保留1位
var numReg1 = /^(([1-9]{1}\d*)|(0{1}))(\.\d{1})?$|^$/;

//正数  最多保留3位
var numReg3 = /^(([1-9]{1}\d*)|(0{1}))(\.\d{1,3})?$|^$/;

//正数  最多保留8位
var numReg8 = /^(([1-9]{1}\d*)|(0{1}))(\.\d{1,8})?$|^$/;
//正数  不保留小数
var numReg = /^(([1-9]{1}\d*)|(0{1}))$|^$/;
//数字+字母
var letterAndNumber = /^\w+$|^$/;
//经度
var lngReg = /^[\-\+]?(0?\d{1,2}\.\d{1,10}|1[0-7]?\d{1}\.\d{1,10}|180\.0{1,10})|^$/;
//纬度
var latReg = /^[\-\+]?([0-8]?\d{1}\.\d{1,10}|90\.0{1,10})|^$/;
//域名验证
var	domainReg = /^(?:(?:[a-zA-Z0-9]|[a-zA-Z0-9][a-zA-Z0-9\-]*[a-zA-Z0-9])\.)+[a-zA-Z]{2,}$/;
var SysValidationTypes = {
		"1":{
			"regx":"numberReg",
			"msg":"错误的数字格式",
		},
		"2":{
			"regx":"mobileReg",
			"msg":"错误的手机号码格式",
		},
		"3":{
			"regx":"emailReg",
			"msg":"错误的电子邮件格式",
		},
		"4":{
			"regx":"ipReg",
			"msg":"错误的IP格式",
		},
		"5":{
			"regx":"urlReg",
			"msg":"错误的URL格式",
		},
		"100":{
			"regx":"emptyReg",
			"msg":"不能为空",
		}
}


window.AndroidClient={};


