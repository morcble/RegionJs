var SwitchPlugin = {
		render:function(region,tmpObj,paraData){
			tmpObj.attr("type","hidden");
			tmpObj.addClass("wrapped");
			var activeText = tmpObj.attr("active-text");
			var inactiveText = tmpObj.attr("inactive-text");
			
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var hiddenInputHtml = tmpObj.prop("outerHTML");;
			if(paraData==null||paraData=="")paraData=0;
			else paraData=parseInt(paraData);
			
			var replaceHtml = '<div class="switch-wrapper region-wrapper">'+hiddenInputHtml;
			if(inactiveText!=null)replaceHtml +='<div class="inactive-text">'+inactiveText+'</div>'
			replaceHtml += '<div class="switch-wrapper-btn">';
			replaceHtml+='<div class="switch-btn"></div>';
			replaceHtml+='</div>';
			if(activeText!=null)replaceHtml +='<div class="active-text">'+activeText+'</div>'
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			var btn_wrapper = replaced.find(".switch-wrapper-btn");
			
			setTimeout(function(){//等待布局计算width
				if(paraData!=null&&paraData!=""&&parseInt(paraData)==1){//初始化值为选中时
					btn_wrapper.addClass("switch-actived");
					var swtichBtn = replaced.find(".switch-btn");
					var width1 = btn_wrapper.width();
					var width2 = swtichBtn.outerWidth(true);
					replaced.find(".switch-btn").css("left",(width1-width2)+'px');
				}
			})
			
			
			btn_wrapper.click(function(event){
				var newVal = 0;
				if(btn_wrapper.hasClass("switch-actived")){
					replaced.find(".switch-btn").animate({left:"0px"},200);
					btn_wrapper.removeClass("switch-actived");
					replaced.find(".switch").val(newVal);
				}
				else{
					btn_wrapper.addClass("switch-actived");
					var swtichBtn = replaced.find(".switch-btn");
					var width1 = btn_wrapper.width();
					var width2 = swtichBtn.outerWidth(true);
					replaced.find(".switch-btn").animate({left:(width1-width2)+'px'},200);
					newVal = 1;
					replaced.find(".switch").val(newVal);
				}
				
				if(onchange!=null&&onchange!=""){
					var indexTag = onchange.indexOf("(");
					if(indexTag==-1){
						eval(onchange+'.call(btn_wrapper,newVal,event)');
					}
					else{
						var script = onchange.substring(0,indexTag)+".call(btn_wrapper,newVal,event"+onchange.substring(indexTag+1,onchange.length);
						eval(script)
					}
				}
				
			});
			
			tmpObj = replaced.find(".switch");
			tmpObj.val(paraData);
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);
			
			var btn_wrapper = wrapperElemObj.find(".switch-wrapper-btn");
			if((paraData+"")=="0"){
				if(btn_wrapper.hasClass("switch-actived")){
					wrapperElemObj.find(".switch-btn").animate({left:"0px"},200);
					btn_wrapper.removeClass("switch-actived");
				}
			}
			else{
				if(!btn_wrapper.hasClass("switch-actived")){
					btn_wrapper.addClass("switch-actived");
					var swtichBtn = wrapperElemObj.find(".switch-btn");
					var width1 = btn_wrapper.width();
					var width2 = swtichBtn.outerWidth(true);
					wrapperElemObj.find(".switch-btn").animate({left:(width1-width2)+'px'},200);
					wrapperElemObj.find(".switch").val(1);
				}
			}
		}
}