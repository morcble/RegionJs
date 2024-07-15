var NumcounterPlugin = {
		render:function(region,tmpObj,paraData){
			var onchange = tmpObj.attr("onchange");
		
			tmpObj.addClass("wrapped");
			tmpObj.addClass("num");
			
			var step = tmpObj.attr("step");
			try{
				step = parseInt(step);
			}catch(e){};
			if(step==null)step=1;
			
			var min = tmpObj.attr("min");
			try{
				min = parseInt(min);
			}catch(e){};
			var max = tmpObj.attr("max");
			try{
				max = parseInt(max);
			}catch(e){};
			
			if(paraData==null||paraData==""){
				paraData=0;
			}
			
			if(paraData<min){
				paraData=min;
			}
			if(paraData>max){
				paraData=max;
			}
			
			var replaceHtml = '<div class="numcounter-wrapper region-wrapper">';
			replaceHtml+='<div class="minus"><i class="fa-solid fa-minus"></i></div>';
			replaceHtml+=tmpObj.prop("outerHTML");
			replaceHtml+='<div class="plus"><i class="fa-solid fa-plus"></i></div>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			replaced.find(".minus").click(function(){
				var oldVal = 0;
				if(min!=null)oldVal=min;
				try{
					oldVal = parseInt(replaced.find(".wrapped").val());
					if(isNaN(oldVal))oldVal=0;
				}catch(e){};
				var newVal = oldVal-step;
				if(min!=null&&newVal<min)newVal=min;
				
				var wrapped = replaced.find(".wrapped");
				wrapped.val(newVal);
				
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(wrapped,newVal)');
					//eval(onchange+"('"+newVal+"')");
				}
			});
			
			replaced.find(".plus").click(function(){
				var oldVal = 0;
				if(min!=null)oldVal=min;
				try{
					oldVal = parseInt(replaced.find(".wrapped").val());
					if(isNaN(oldVal))oldVal=0;
				}catch(e){};
				var newVal = oldVal+step;
				if(max!=null&&newVal>max)newVal=max;

				var wrapped = replaced.find(".wrapped");
				wrapped.val(newVal);
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(wrapped,newVal)');
					//eval(onchange+"('"+newVal+"')");
				}
			});
			
			tmpObj = replaced.find(".wrapped");
			
			tmpObj.on("input",function(){
				var newVal = null;
				var wrapped = replaced.find(".wrapped");
				try{
					newVal = parseInt(wrapped.val());
					if(min>newVal){
						newVal=min;
						wrapped.val(newVal);
					}
				}catch(e){};
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(wrapped,newVal)');
					//eval(onchange+"('"+newVal+"')");
				}
			});
			
			
			tmpObj.bind("input",function(){
				var newVal=this.value.replace(/[^\d]/g,'');
				if(max!=null&&newVal>max){
					this.value = max;
				}
				else this.value = newVal;
			});
			tmpObj.val(paraData);
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			regionElemObj.val(paraData);
		}
}