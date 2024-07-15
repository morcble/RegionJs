var DropdownPlugin = {
		render:function(region,selectObj,paraData){
			var onchange = selectObj.attr("onchange");
			if(onchange==null)onchange="";
			
			var className = selectObj[0].className;
			
			var isDisabled = selectObj.hasClass("disabled");
			
			//var editable = selectObj.hasClass("region-editable");
			var searchAble = selectObj.hasClass("region-searchable");
			
			var width = selectObj.attr("width");
			var isInner = selectObj.hasClass("inner");
			
			selectObj.addClass("wrapped");
			var regionDsName = selectObj.attr("region-ds");
			var options = RegionUtil.getRegionDsList(region,regionDsName);

			var selectedVal = paraData;//当前值
			if(selectedVal==null)selectedVal="";
			var optionsObject = DropdownPlugin.getOptionsObject(options,selectedVal,selectObj,null);

			var optionsHtml = optionsObject.optionsHtml;
			var valueDesc = optionsObject.selectedValueDesc;
			
			var regionAttr = selectObj.attr("region-attr");
			
			var uuid = RegionUtil.getUUID();
			var codeTemplate = "<div class=\"dropdown-wrapper region-wrapper\"><input type=\"hidden\" onchange=\""+onchange+"\" region-ds=\""+regionDsName+"\" region-attr=\""+regionAttr;
			if(searchAble)
				codeTemplate+="\" class=\"wrapped region-searchable\">";
			else 
				codeTemplate+="\" class=\"wrapped\">";
			codeTemplate+="<input type=\"text\" readonly=\"readonly\" class=\"shown form-control\"><div class=\"select-icon\"><i class=\"fa-solid fa-chevron-down\"></i></div><ul class=\"options messageoptions\" style=\"display:none;width:max-content;\">";
			codeTemplate+=optionsHtml;
			codeTemplate+="</ul></div>";
			var replaced = $(codeTemplate);
			
			replaced.insertAfter(selectObj);
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			if(isDisabled){
				replaced.addClass("disabled");
				replaced.find(".shown").addClass("disabled");
			}
			
			
			selectObj.remove();
			
			if(width!=null){
				replaced.css("width",width);
				replaced.find("input[type='text']").css("width",width);
			}
			
			setTimeout(function(){
				var tmpWidth = replaced.find("input[type='text']").outerWidth(true);
				if(tmpWidth==0)tmpWidth=replaced.css("width");
				replaced.find("ul").css("min-width",tmpWidth);
			});
			
	
			replaced.resize(function(){
				replaced.find(".messageoptions").css("min-width",replaced.find("input[type='text']").outerWidth(true));
			});

			var targetInput = replaced.find(".wrapped");//实际存值的地方
			targetInput.val(selectedVal);
			
			var selectWrapper = replaced;
			if(valueDesc!=null)
				selectWrapper.find("input[type='text']").val(valueDesc);//设置默认选中值
				
			selectWrapper.attr("wrapper-id",uuid);
			
			//如果当前region是grid的内部region
			var targetRegion = region;
			if(region.isInGrid){
				targetRegion = region.parent;
			}
			if(targetRegion.selectFunctions==null)
				targetRegion.selectFunctions = {};
			targetRegion.selectFunctions[uuid+"_readyHideMenu"] = function(){
				setTimeout(function(){
					var ot = selectWrapper.attr("ot");
					var oc = selectWrapper.attr("oc");
					if((ot==null||ot==0)&&(oc==null||oc==0)){
						selectWrapper.find("ul").slideUp(100);
						selectWrapper.attr("shown",0);
					}
				},20);
			};
			
			DropdownPlugin.addEventListenerForOptions(selectWrapper);
			
			selectWrapper.click(function(){
				//判断位置
				var regionDsName = selectObj.attr("region-ds");
				var options = RegionUtil.getRegionDsList(region,regionDsName);
				
				var optionsHeight = 34.5*options.length;//大概的高度
				if(optionsHeight>185)optionsHeight=185;
				
				var optionsPostionType = 1;//1 bottom ,2  top
				var event = window.event || arguments.callee.caller.arguments[0];
				
				var coverWindow = selectWrapper.closest(".cover-window");
				var bottomGap;
				var topGap;
				if(coverWindow.length==1){//在弹窗内
					bottomGap = (coverWindow.outerHeight(true)+coverWindow[0].offsetTop-event.clientY-selectWrapper.outerHeight(true));
					topGap = RegionUtil.getDomPosition(selectWrapper[0]).y - RegionUtil.getDomPosition(coverWindow[0]).y;
				}
				else{
					bottomGap = (document.body.clientHeight-event.clientY-selectWrapper.outerHeight(true));
					topGap = RegionUtil.getDomPosition(selectWrapper[0]).y;
				}
				
				if(topGap<bottomGap)bottomGap=optionsHeight;//显示在下方
				
				if(bottomGap<optionsHeight){//判断是否超过浏览器可视边界
					optionsPostionType = 2;
				}
			
				var shown = selectWrapper.attr("shown");
				if(shown==null||shown==0){
					selectWrapper.attr("ot",1);
					selectWrapper.attr("shown",1);
					if(optionsPostionType==2){//底部不够显示options
						DropdownPlugin.showOptionsAtTop(selectWrapper,optionsHeight);
					}
					else{
						DropdownPlugin.showOptionsAtBottom(selectWrapper,optionsHeight);
					}
					
				}
				else{
					selectWrapper.attr("ot",0);
					targetRegion.selectFunctions[selectWrapper.attr("wrapper-id")+"_readyHideMenu"]();
				}
			});
	
			selectWrapper.find("input[type='text']").mouseover(function(){
				if(selectWrapper.attr("shown")==1)
					selectWrapper.attr("ot",1);
			});

			selectWrapper.find("input[type='text']").mouseout(function(){
				if(selectWrapper.attr("shown")==1){
					selectWrapper.attr("ot",0);
					targetRegion.selectFunctions[selectWrapper.attr("wrapper-id")+"_readyHideMenu"]();
				}	
			});
			
			selectWrapper.find(".options").mouseover(function(){
				selectWrapper.attr("oc",1);
				selectWrapper.attr("ot",0);
			});
			
			selectWrapper.find(".options").mouseout(function(){
				selectWrapper.attr("oc",0)
				targetRegion.selectFunctions[selectWrapper.attr("wrapper-id")+"_readyHideMenu"]();
			}); 
			if(isInner)selectWrapper.find(".wrapped").addClass("inner");
			return selectWrapper.find(".wrapped");
		},
		//options显示在上方
		showOptionsAtTop:function(selectWrapper,optionsHeight){
			var ulObj = selectWrapper.find("ul");
			if(!ulObj.hasClass("option-top")){
				selectWrapper.find("li").remove().each(function(){
					ulObj.prepend($(this));
				});
				ulObj.addClass("option-top");
				ulObj.css("top",-optionsHeight-5);
				DropdownPlugin.addEventListenerForOptions(selectWrapper);
			}
			ulObj.show(100);
		},
		//options显示在下方
		showOptionsAtBottom:function(selectWrapper,optionsHeight){
			var ulObj = selectWrapper.find("ul");
			if(ulObj.hasClass("option-top")){
				selectWrapper.find("li").remove().each(function(){
					ulObj.prepend($(this));
				});
				ulObj.removeClass("option-top");
				ulObj.css("top",selectWrapper.outerHeight(true));
				
				DropdownPlugin.addEventListenerForOptions(selectWrapper);
			}
			ulObj.slideDown(100);
		},
		addEventListenerForOptions:function(selectWrapper){
			var targetInput = selectWrapper.find(".wrapped");
			var onchange = targetInput.attr("onchange");
			
			var region =RegionUtil.getRegionByElem(selectWrapper[0]);
			selectWrapper.find("li").click(function(){
				//选中后的操作
				selectWrapper.find("li").removeClass("selected");
				$(this).addClass("selected");
				selectWrapper.attr("oc",0)
				
				region.selectFunctions[selectWrapper.attr("wrapper-id")+"_readyHideMenu"]();
				
				var currentVal = targetInput.val();//旧值   ~~~~~
				var newVal = this.getAttribute("val");//新值
				if(currentVal!=newVal){
					var input = selectWrapper.find("input[type='text']");
					input.val($(this).text())
					targetInput.val(this.getAttribute("val"));
					if(onchange!=null&&onchange!=""){
						eval(onchange+'.call(this,newVal)');
						//eval(onchange+"('"+newVal+"')");
					}
				}
			});
		},
		getOptionsObject:function(options,selectedVal,regionElemObj,selectWrapper){
			var selectedValueDesc = null;
			var optionsHtml = '';
			
			regionElemObj.find("option").each(function () {
	            if(selectedVal==$(this).val()){
	            	optionsHtml+="<li class='selected' val='"+$(this).val()+"'><font class='label-txt'>"+$(this).text()+"</font></li>"; 
	            	selectedValueDesc=$(this).text();
	            }
	            else{
	            	optionsHtml+="<li val='"+$(this).val()+"'><font class='label-txt'>"+$(this).text()+"</font></li>"; 
	            }
	        });
			
			if(selectWrapper!=null){
				if(selectedVal==null||selectedVal==""){
					optionsHtml+="<li class='selected' val=''><font class='label-txt'>请选择</font></li>";
					selectedValueDesc="请选择";
				}
				else{
					optionsHtml+="<li val=''><font class='label-txt'>请选择</font></li>";
				}
			}
			
			if(options!=null){
				for(var i = 0 ; i <options.length;i++){
					if(selectedVal==options[i].value){
						optionsHtml+="<li class='selected' val='"+options[i].value+"'><font class='label-txt'>"+options[i].text+"</font></li>";
						selectedValueDesc=options[i].text;
					}
					else{
						optionsHtml+="<li val='"+options[i].value+"'><font class='label-txt'>"+options[i].text+"</font></li>";
					}
				}
			}
			if(selectWrapper!=null){
				if(selectedValueDesc!=null){
					selectWrapper.find("input[type='text']").val(selectedValueDesc);//设置默认选中值
				}
				else{
					selectWrapper.find("input[type='text']").val("请选择");
				}
			}
			
			return {optionsHtml:optionsHtml,selectedValueDesc:selectedValueDesc};
		},
		refreshDataSource:function(region,wrapperElemObj,regionElemObj,options){//动态更新options
			var selectedVal = regionElemObj.val();
			var selectedValueDesc = null;

			var selectWrapper = wrapperElemObj;
			var optionsObject = DropdownPlugin.getOptionsObject(options,selectedVal,regionElemObj,wrapperElemObj);
			
			var optionsHtml = optionsObject.optionsHtml;
			var valueDesc = optionsObject.selectedValueDesc;
			
			selectWrapper.find(".options").html(optionsHtml);
			RegionUtil.addRegionUniqueId(selectWrapper[0],region.regionId);
			DropdownPlugin.addEventListenerForOptions(selectWrapper);
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			if(paraData==null)paraData="";
			else paraData+="";
			regionElemObj.val(paraData);
			
			wrapperElemObj.find(".options").find("li").each(function(){
				if(paraData == $(this).attr("val")){
					var input = wrapperElemObj.find("input[type='text']");
					input.val($(this).find(".label-txt").text());
					return false;//结束循环
				}
			});
		}
}