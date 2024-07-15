var StarPlugin = {
		render:function(region,tmpObj,paraData){
			var maxScore = tmpObj.attr("max-score");
			if(maxScore==null)maxScore=5;
			else maxScore = parseInt(maxScore);
			tmpObj.addClass("wrapped");
			tmpObj.attr("type","hidden");
			
			var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var hiddenInputHtml = tmpObj.prop("outerHTML");;
			
			var starNum = 0;
			if(paraData==null)starNum=0;
			else starNum=parseInt(paraData);
			
			var replaceHtml = '<div class="star-wrapper region-wrapper">'+hiddenInputHtml;
			for(var i=0;i<maxScore;i++,starNum--){
				if(starNum>0)replaceHtml+='<i class="fa fa-star" num="'+i+'"></i>';
				else replaceHtml+='<i class="fa fa-light fa-star" num="'+i+'"></i>';
			}
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			replaced.find("i").click(function(){
				var newScore = $(this).attr("num");
				newScore=parseInt(newScore)+1;
				var starsArray = replaced.find("i");
				replaced.find(".star").val(newScore);
				for(var i=0;i<starsArray.length;i++,newScore--){
					if(newScore>0){
						$(starsArray[i]).removeClass("fa-light fa-star");
						$(starsArray[i]).addClass("fa-star");
					}
					else{
						$(starsArray[i]).removeClass("fa-star");
						$(starsArray[i]).addClass("fa-light fa-star");
					}
				}
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(replaced,newScore)');
					//eval(onchange+"('"+newVal+"')");
				}
			});
			
			tmpObj = replaced.find(".star");
			tmpObj.val(paraData);
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);
			var starsArray = wrapperElemObj.find("i");
			var newScore = paraData;
			for(var i=0;i<starsArray.length;i++,newScore--){
				if(newScore>0){
					$(starsArray[i]).removeClass("fa-light fa-star");
					$(starsArray[i]).addClass("fa-star");
				}
				else{
					$(starsArray[i]).removeClass("fa-star");
					$(starsArray[i]).addClass("fa-light fa-star");
				}
			}
		}
}