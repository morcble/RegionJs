var SliderPlugin = {
		render:function(region,tmpObj,paraData){
			tmpObj.addClass("wrapped");
			tmpObj.attr("type","hidden");
	
			if(paraData==null||paraData==""){
				paraData=0;
			}

			var hiddenInputHtml = tmpObj.prop("outerHTML");
			var width = tmpObj.attr("width");
			if(width==null)width="100px";
			
			var replaceHtml = '<div class="slider-wrapper region-wrapper">'+hiddenInputHtml;
			replaceHtml+='<div class="bar-container"><div class="bar-actived"></div><div class="bar-control"></div></div>';
			replaceHtml+='<div class="val-prompt"></div>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
	
			var barControl = replaced.find(".bar-control");
			replaced.css("width",width);
			
			setTimeout(function(){
				SliderPlugin.renderWithVal(replaced,paraData);
			});
			
			
			replaced.find(".bar-container").click(function(event){
				var offset = event.clientX-(RegionUtil.getDomPosition(barControl[0]).x+10);
				
				var activedWidth = replaced.find(".bar-actived").css("width");
				activedWidth = parseInt(activedWidth.substring(0,activedWidth.length-2));
				
				var newWidth = activedWidth+offset;
				replaced.find(".bar-actived").css("width",(newWidth)+"px");
				barControl.css("left",(newWidth-8)+"px");//8 width
				
				SliderPlugin.calculateVal(replaced);
			});

			barControl.mousedown(function(event){
				var control = $(this);
				control.attr("start",event.clientX);
				control.attr("startWidth",replaced.find(".bar-actived").css("width"));

				var maxWidth = replaced.find(".bar-container").css("width");
				maxWidth = parseInt(maxWidth.substring(0,maxWidth.length-2));
				
				replaced.mousemove(function(evt){
					var offset = evt.clientX - parseInt(control.attr("start"));
					var startWidth = control.attr("startWidth");
					startWidth = parseInt(startWidth.substring(0,startWidth.length-2));
					
					var newWidth = startWidth+offset;
					if(newWidth>maxWidth){
						newWidth = maxWidth;
					}
					else if(newWidth<0){
						newWidth = 0;
					}
					replaced.find(".bar-actived").css("width",(newWidth)+"px");
					control.css("left",(newWidth-8)+"px");//8 width
					
					SliderPlugin.calculateVal(replaced);
				});
				
				$(region.getRegionDivElem()).mouseout(function(event){
					if(event.fromElement==region.getRegionDivElem()){
						control.attr("start",null);
						control.attr("startWidth",null);
						replaced.unbind("mousemove");
						$(region.getRegionDivElem()).unbind("mouseout");
						$(region.getRegionDivElem()).unbind("mouseup");

						SliderPlugin.calculateVal(replaced);
					}
					
				});
				$(region.getRegionDivElem()).mouseup(function(event){
					control.attr("start",null);
					control.attr("startWidth",null);
					replaced.unbind("mousemove");
					$(region.getRegionDivElem()).unbind("mouseout");
					$(region.getRegionDivElem()).unbind("mouseup");

					SliderPlugin.calculateVal(replaced);
				});
			});

			tmpObj = replaced.find(".wrapped");
			tmpObj.val(paraData);
			
			replaced.find(".val-prompt").text(paraData);
			return tmpObj;
		},
		/**
		 * 根据值 更新界面和数据
		 * width 控件宽度
		 */
		renderWithVal:function(wrapperElemObj,curVal){
			var tmpObj = wrapperElemObj.find(".wrapped")
			var min = tmpObj.attr("min");
			try{
				min = parseInt(min);
			}catch(e){};
			if(min==null)min=0;
			
			var max = tmpObj.attr("max");
			try{
				max = parseInt(max);
			}catch(e){};
			if(max==null)max=100;
	
			if(curVal<min)curVal=min;
			else if(curVal>max)curVal=max;
			
			var width = wrapperElemObj.find(".bar-container").width();
			var gapUnit = width/(max-min);//每个单位占用多少css宽度
			var barControl = wrapperElemObj.find(".bar-control");
			var startPosition = gapUnit*(curVal-min);
			wrapperElemObj.find(".bar-actived").css("width",startPosition);
			barControl.css("left",startPosition-barControl.outerWidth(true)/2-2);
		},
		/**
		 * 根据位置计算值
		 */
		calculateVal:function(wrapperElemObj){
			var tmpObj = wrapperElemObj.find(".wrapped")
			var min = tmpObj.attr("min");
			try{
				min = parseInt(min);
			}catch(e){};
			if(min==null)min=0;
			
			var max = tmpObj.attr("max");
			try{
				max = parseInt(max);
			}catch(e){};
			if(max==null)max=100;
			
			var maxWidth = wrapperElemObj.find(".bar-container").css("width");
			maxWidth = parseInt(maxWidth.substring(0,maxWidth.length-2));
			
			var curWidth = wrapperElemObj.find(".bar-actived").css("width");
			curWidth = parseInt(curWidth.substring(0,curWidth.length-2));

			var curVal = (curWidth/maxWidth)*(max-min)+min;
			curVal=Math.round(curVal);
			tmpObj.val(curVal);
			SliderPlugin.renderWithVal(wrapperElemObj,curVal);
			
			wrapperElemObj.find(".val-prompt").text(curVal);
			
			var onchange = tmpObj.attr("onchange");
			if(onchange!=null&&onchange!=""){
				eval(onchange+'.call(tmpObj,curVal)');
			}
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);
			
		}
}