var MultiDropdownPlugin = {
		render:function(region,selectObj,paraData){
			var onchange = selectObj.attr("onchange");
			if(onchange==null)onchange="";
			
			
			var placeholder = selectObj.attr("placeholder");

			var isDisabled = selectObj.hasClass("disabled");
			//var editable = selectObj.hasClass("region-editable");
			var searchAble = selectObj.hasClass("region-searchable");
			
			var width = selectObj.attr("width");
			var isInner = selectObj.hasClass("inner");
			
			var regionDsName = selectObj.attr("region-ds");
			var options = RegionUtil.getRegionDsList(region,regionDsName);

			if(paraData==null)paraData="";
			else paraData=paraData+"";
			
			var optionsHtml = "";
			
			if(paraData.endWith(",")){
				paraData=paraData.substring(0,paraData.length-1);
			}
			var currentValueArray = null;
			if(paraData!="")
				currentValueArray = paraData.split(",");
			
			selectObj.find("option").each(function () {
	            optionsHtml+="<li val='"+$(this).val()+"'><input class='checkitem' type=\"checkbox\"><font class='label-txt'>"+$(this).text()+"</font></li>"; 
	        })
			
			if(options!=null){
				for(var i = 0 ; i <options.length;i++){
					optionsHtml+="<li val='"+options[i].value+"'><input class='checkitem' type=\"checkbox\"><font class='label-txt'>"+options[i].text+"</font></li>";
				}
			}
			
			var regionAttr = selectObj.attr("region-attr");
			
			var uuid = RegionUtil.getUUID();
			var codeTemplate = "<div class=\"m-dropdown-wrapper region-wrapper\"><input type=\"hidden\" onchange=\""+onchange+"\" region-ds=\""+regionDsName+"\" region-attr=\""+regionAttr;
			if(searchAble)
				codeTemplate+="\" class=\"wrapped region-searchable\">";
			else 
				codeTemplate+="\" class=\"wrapped\">";
			
			codeTemplate+="<div readonly=\"readonly\" class=\"item-container\"><div class=\"selectedItems shown\">   </div></div>";
			codeTemplate+="<div class=\"select-icon-wrap\"><div class=\"select-icon\"><i class=\"fa-solid fa-lg fa-chevron-down\"></i></div></div><ul class=\"options messageoptions\" style=\"display:none\">";
			codeTemplate+=optionsHtml;
			codeTemplate+="</ul></div>";
			var replaced = $(codeTemplate);
		
			replaced.insertAfter(selectObj);
			if(isDisabled){
				replaced.addClass("disabled");
				replaced.find(".shown").addClass("disabled");
			}
			var selectWrapper = replaced;
			
			if(placeholder!=null)selectWrapper.attr("placeholder",placeholder);
			
			setTimeout(function(){
				//var tmpWidth =selectWrapper.css("width");
				if(width!=null)selectWrapper.find(".item-container").css("width",width);
			});
			
			//初始化有值的多选下拉列表
			if(currentValueArray!=null&&currentValueArray.length>0){
				var valueCache = {};
				for(var i=0;i<currentValueArray.length;i++){
					valueCache[currentValueArray[i]] = true;
				}
				//下拉列表checkbox 选中
				var initCheckItems = selectWrapper.find(".checkitem");
				var selectedItems = selectWrapper.find(".selectedItems");
				
				if(placeholder!=null){
					var selectedItems = selectWrapper.find(".selectedItems");
					selectedItems.html("<div class='placeholder hidden'>"+placeholder+"</div>");
				}
				else{
					selectedItems.html("");
				}
				initCheckItems.each(function(){//代码类似1
					var initCheckItem = $(this);
					var tmpLiobj = initCheckItem.parent();
					var value = tmpLiobj.attr("val");
					if(valueCache[value]){
						initCheckItem.prop("checked",true);
				
						//增加可关闭的item
						var labelTxt = tmpLiobj.find(".label-txt").text();
						selectedItems.append("<div val='"+value+"' class=\"value_"+value+" selected\">"+labelTxt+"<div class=\"itemclose\">&nbsp;</div></div>");
						
						selectWrapper.find(".itemclose").click(function(evt){
							var parentDivObj = $(this).parent();
							var tmpValue = (parentDivObj.attr("val"))
							
							selectWrapper.find(".options").find("li").each(function(){
								if(tmpValue==$(this).attr("val")){
									$(this).find(".checkitem").prop("checked",false);
									return false;
								}
							});
							
							parentDivObj.remove();
							
							var newValue = MultiDropdownPlugin.calculateValFunc(selectWrapper);
							
							if(onchange!=null&&onchange!=""){
								eval(onchange+'.call(this,newValue)');
								//eval(onchange+"('"+newValue+"')");
							}
							evt.stopPropagation();
						});
						//end
					}
				});
			}
			else{
				if(placeholder!=null){
					var selectedItems = selectWrapper.find(".selectedItems");
					selectedItems.html("<div class='placeholder'>"+placeholder+"</div>");
				}	
			}

			//初始化有值的多选下拉列表 end
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			selectObj.remove();
			
			
			if(width!=null){
				selectWrapper.css("width",width);
			}
			
			var targetInput = selectWrapper.find(".wrapped");//实际存值的地方
			targetInput.val(paraData);
				
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
						selectWrapper.find("ul").slideUp(200);
						selectWrapper.attr("shown",0);
					}
				},20);
			};
			
			MultiDropdownPlugin.addEventListenerForOptions(selectWrapper);

			selectWrapper.click(function(){
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
						MultiDropdownPlugin.showOptionsAtTop(selectWrapper,optionsHeight);
					}
					else{
						MultiDropdownPlugin.showOptionsAtBottom(selectWrapper,optionsHeight);
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
			
			selectWrapper.find(".item-container").mouseout(function(){
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
		relocateIcon:function(selectWrapper){
			setTimeout(function(){
				selectWrapper.find(".select-icon-wrap").css("line-height",selectWrapper.outerHeight()+"px");
			});
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
				MultiDropdownPlugin.addEventListenerForOptions(selectWrapper);
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
				
				MultiDropdownPlugin.addEventListenerForOptions(selectWrapper);
			}
			ulObj.slideDown(100);
		},
		addEventListenerForOptions:function(selectWrapper){
			selectWrapper.find(".checkitem").click(function(){
				var onchange = selectWrapper.find(".wrapped").attr("onchange");
				var ischecked = $(this).prop("checked");
				var selectedItems = selectWrapper.find(".selectedItems");
				var liObj = $(this).closest("li");
				var value = liObj.attr("val");
				if(ischecked){
					var labelTxt = liObj.find(".label-txt").text();
					selectedItems.append("<div val='"+value+"' class=\"value_"+value+" selected\">"+labelTxt+"<div class=\"itemclose\">&nbsp;</div></div>");
					
					selectWrapper.find(".itemclose").click(function(evt){
						var parentDivObj = $(this).parent();
						var tmpValue = (parentDivObj.attr("val"))
						
						selectWrapper.find(".options").find("li").each(function(){
							if(tmpValue==$(this).attr("val")){
								$(this).find(".checkitem").prop("checked",false);
								return false;
							}
						});
						
						parentDivObj.remove();
						
						var newValue = MultiDropdownPlugin.calculateValFunc(selectWrapper);
						
						if(onchange!=null&&onchange!=""){
							eval(onchange+'.call(this,newValue)');
							//eval(onchange+"('"+newValue+"')");
						}
						evt.stopPropagation();
					});
				}
				else{
					selectedItems.find(".value_"+value).remove();
				}
				
				var newValue = MultiDropdownPlugin.calculateValFunc(selectWrapper);
				
				if(onchange!=null&&onchange!=""){
					eval(onchange+'.call(this,newValue)');
					//eval(onchange+"('"+newValue+"')");
				}
			});
		},
		//计算选中的值 并赋予input text
		calculateValFunc:function(selectWrapper){
			var newValue = "";
			selectWrapper.find(".checkitem").each(function(){
				if($(this).prop("checked")){
					if(newValue.length==0)
						newValue+=$(this).closest("li").attr("val");
					else
						newValue+=","+$(this).closest("li").attr("val");
				}
			});
			selectWrapper.find(".wrapped").val(newValue);
			
			var itemsCount = selectWrapper.find(".selectedItems>.selected").length;
			if(itemsCount==0){
				selectWrapper.find(".selectedItems>.placeholder").removeClass("hidden");
			}
			else{
				selectWrapper.find(".selectedItems>.placeholder").addClass("hidden");
			}
			
			MultiDropdownPlugin.relocateIcon(selectWrapper);
			return newValue;
		},
		refreshDataSource:function(region,wrapperElemObj,regionElemObj,options){//动态更新options
			var paraData = regionElemObj.val();
			var onchange = regionElemObj.attr("onchange");
			
			var placeholder = wrapperElemObj.attr("placeholder");
			
			var selectWrapper = wrapperElemObj;
			
			var optionsHtml ="";
			if(paraData.endWith(",")){
				paraData=paraData.substring(0,paraData.length-1);
			}
			
			var currentValueArray = null;
			if(paraData!="")
				currentValueArray = paraData.split(",");

			if(options!=null){
				for(var i = 0 ; i <options.length;i++){
					optionsHtml+="<li val='"+options[i].value+"'><input class='checkitem' type=\"checkbox\"><font class='label-txt'>"+options[i].text+"</font></li>";
				}
			}
			selectWrapper.find(".options").html(optionsHtml);
			RegionUtil.addRegionUniqueId(selectWrapper[0],region.regionId);
			//初始化有值的多选下拉列表
			if(currentValueArray!=null&&currentValueArray.length>0){
				var valueCache = {};
				for(var i=0;i<currentValueArray.length;i++){
					valueCache[currentValueArray[i]] = true;
				}
				//下拉列表checkbox 选中
				var initCheckItems = selectWrapper.find(".checkitem");
				var selectedItems = selectWrapper.find(".selectedItems");
				selectedItems.html("");
				initCheckItems.each(function(){//代码类似1
					var initCheckItem = $(this);
					var tmpLiobj = initCheckItem.parent();
					var value = tmpLiobj.attr("val");
					if(valueCache[value]){
						initCheckItem.prop("checked",true);
				
						//增加可关闭的item
						var labelTxt = tmpLiobj.find(".label-txt").text();
						selectedItems.append("<div val='"+value+"' class=\"value_"+value+" selected\">"+labelTxt+"<div class=\"itemclose\">&nbsp;</div></div>");
						
						selectWrapper.find(".itemclose").click(function(evt){
							var parentDivObj = $(this).parent();
							var tmpValue = (parentDivObj.attr("val"))
							
							selectWrapper.find(".options").find("li").each(function(){
								if(tmpValue==$(this).attr("val")){
									$(this).find(".checkitem").prop("checked",false);
									return false;
								}
							});
							
							parentDivObj.remove();
							
							var newValue = MultiDropdownPlugin.calculateValFunc(selectWrapper);
							
							if(onchange!=null&&onchange!=""){
								eval(onchange+'.call(this,newValue)');
								//eval(onchange+"('"+newValue+"')");
							}
							evt.stopPropagation();
						});
						//end
					}
				});
			}
			else{
				if(placeholder!=null){
					var selectedItems = selectWrapper.find(".selectedItems");
					selectedItems.html("<div class='placeholder'>"+placeholder+"</div>");
				}	
			}
			
			MultiDropdownPlugin.addEventListenerForOptions(selectWrapper);
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			if(paraData==null)paraData="";
			regionElemObj.val(paraData);
			
			var regionDsName = regionElemObj.attr("region-ds");
			var options = RegionUtil.getRegionDsList(region,regionDsName);
			MultiDropdownPlugin.refreshDataSource(region,wrapperElemObj,regionElemObj,options);
		}
}