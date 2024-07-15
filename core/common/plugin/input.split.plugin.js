//分割输入框  		selectedItemArray = [{"text":"组件字段的数据类型","value":"4713683520248086528"},{"text":"组件数据结构","value":"4713632394031136768"}];
var SplitInputPlugin = {
		render:function(region,tmpObj,paraData){
		/*	var onchange = tmpObj.attr("onchange");
			if(onchange==null)onchange="";*/
			
			var onquery = tmpObj.attr("onquery");
			if(onquery==null)onquery="";
			
			var placeholder = tmpObj.attr("placeholder");
			if(placeholder==null)placeholder="";
			
			var dialogRs = tmpObj.attr("split-dialog");
			if(dialogRs==null)dialogRs = "common/shared/float-dialog/multupleChoose.rs";//默认使用的dialog rs
			
			var isDisabled = tmpObj.hasClass("disabled");
			if(isDisabled){
				tmpObj.attr("readonly","readonly");
			}
			
			if(region.splitDialogCache==null){
				region.splitDialogCache = {};
			}
			var regionAttr = tmpObj.attr("region-attr");
			region.splitDialogCache[regionAttr] = {};
			
			var hide = tmpObj.attr("hide");
			if("true"==hide){
				tmpObj.closest(".data-block-wrapper").addClass("hidden");
			}
			
			var tail = tmpObj.attr("tail");
			
			tmpObj.addClass("wrapped");
			var width = tmpObj.attr("width");
			if(width!=null){
				tmpObj.css("width",width);
			}
			
			var selectedItemArray = [];
			if(paraData!=null&&paraData!=""){
				try{
					var parsedObj = JSON.parse(paraData);
					if(parsedObj instanceof Array){
						selectedItemArray = parsedObj;
					}
				}
				catch(e){console.log("invalid data format")};
			}
			region.splitDialogCache[regionAttr].selectedItemArray = selectedItemArray;
			tmpObj.addClass("hidden");
			

			var replaceHtml = '<div class="split-input-wrapper region-wrapper">'+tmpObj.prop("outerHTML")+'<input type="text" placeholder="'+placeholder+'" class="shadow form-control">';
			replaceHtml+='		<div class="items">';
			
			for(var i = 0 ; i<selectedItemArray.length;i++){
				replaceHtml+='			<div class="item" value="'+selectedItemArray[i].value+'"><span class="item-text">'+selectedItemArray[i].text+'</span><i class="fa-solid fa-remove"></i></div>';
			}
			replaceHtml+='		</div>';
			replaceHtml+='</div>';
			
			var wrapped = $(replaceHtml);
			if(width!=null){
				wrapped.css("width",width);
			}
			if(tail!=null)wrapped.find(".text-tail").html(tail);
			
			RegionUtil.addRegionUniqueId(wrapped[0],region.regionId);
			tmpObj.replaceWith(wrapped);
			
			region.splitDialogCache[regionAttr].wrapped = wrapped;
			
			setTimeout(function(){
				wrapped.find(".shadow").css("padding-left",wrapped.find(".items").width()+8);//设置左边宽度
			})
			wrapped.find(".item>.fa-remove").click(function(){
				SplitInputPlugin.onRemoveClick(this);
			})
			
			
			if(onquery!=null&&onquery.trim()!=""){
				var shadowInput = wrapped.find(".shadow");
				shadowInput.focus(function(){
					if(shadowInput.parent().hasClass("disabled"))return;
					
					if(region.splitDialogCache[regionAttr].msgDialogRegion!=null)return;
					
					if(shadowInput.closeOptionsTask!=null)clearTimeout(shadowInput.closeOptionsTask);
					
					subSearch(shadowInput);
				});
				
				shadowInput.blur(function(){//失焦关闭
					if(shadowInput.parent().hasClass("disabled"))return;
					var dialogRegion = region.splitDialogCache[regionAttr].msgDialogRegion;
					if(dialogRegion==null)return;
					
					var optionRegionRootObj = $(dialogRegion.getRegionDivElem());
					var overFlag = optionRegionRootObj.attr("ov");
					if(overFlag=="1")return;//如果鼠标在弹窗上 则返回
					
					if(shadowInput.closeOptionsTask!=null)clearTimeout(shadowInput.closeOptionsTask);
					if(wrapped.searchTask!=null)clearTimeout(wrapped.searchTask);//避免延迟查询造成UI错位

					RS.clearMsg(dialogRegion.regionId);
					region.splitDialogCache[regionAttr].msgDialogRegion = null;
				});
				
				shadowInput.on("input",function(){
					if(shadowInput.parent().hasClass("disabled")){
						shadowInput.val("");
						return;
					}
					
					var inputElem = this;
					if(wrapped.searchTask!=null){
						clearTimeout(wrapped.searchTask);//避免频繁查询
					}
					wrapped.searchTask = setTimeout(function(){
						subSearch(shadowInput);
					},300);
				});
				
				function subSearch(shadowInput){
					var curText = shadowInput.val();
					var onqueryPromise = eval(onquery+'.call(shadowInput[0],curText)');
					if(onqueryPromise==null)return;
					onqueryPromise.done(function(queryResult){
						if(region.splitDialogCache[regionAttr].msgDialogRegion==null){
							var domPosition = RS.getDomPosition(shadowInput[0]);
							var msgDialogPromise = RS.showMsg(shadowInput[0],dialogRs,null,domPosition.h+8,0);
							msgDialogPromise.done(function(msgDialogRegion){
								$(msgDialogRegion.getRegionDivElem()).css("width",domPosition.w+"px");
								region.splitDialogCache[regionAttr].msgDialogRegion = msgDialogRegion;
								msgDialogRegion.renderDialogData(queryResult,region.splitDialogCache[regionAttr].selectedItemArray);
								
								msgDialogRegion.onClickItem = function(checked,impactedValue,impactedText,allowMultiple){
									if(checked){//清除输入框，并加入新选项
										var tmpObj = {};
										tmpObj.value = impactedValue;
										tmpObj.text = impactedText;
										if(!allowMultiple){
											region.splitDialogCache[regionAttr].selectedItemArray = [];//单选需要清空数组
										}
										region.splitDialogCache[regionAttr].selectedItemArray.push(tmpObj);
										
										
										var tmpItemHtml = '<div class="item" value="'+impactedValue+'"><span class="item-text">'+impactedText+'</span><i class="fa-solid fa-remove"></i></div>';
										if(!allowMultiple){
											RS.setInnerHtml(region,wrapped.find(".items")[0],tmpItemHtml);
										}
										else{
											RS.appendHtml(region,wrapped.find(".items")[0],tmpItemHtml);
										}
										wrapped.find(".item:last-child>.fa-remove").click(function(){
											SplitInputPlugin.onRemoveClick(this);
										})
										
										shadowInput.val("");
									}
									else{//删除选项
										var resultValArray = [];
										wrapped.find(".item").each(function(){
											//移除和计算新值
											var tmpVal = this.getAttribute("value");
											if(tmpVal==impactedValue){
												$(this).remove();
											}
											else{
												var tmpObj = {};
												tmpObj.value = this.getAttribute("value");
												tmpObj.text = $(this).children(".item-text").text();
												resultValArray.push(tmpObj);
											}
										})
										region.splitDialogCache[regionAttr].selectedItemArray = resultValArray;//重新赋值
									}
									//隐藏赋值,重新适配输入框左侧起始位置
									var newValue = JSON.stringify(region.splitDialogCache[regionAttr].selectedItemArray);
									wrapped.find(".wrapped").val(newValue);//TOTO
									wrapped.find(".shadow").css("padding-left",wrapped.find(".items").width()+8).focus();
									
									SplitInputPlugin.onchangeCallBack(wrapped.find(".wrapped"),newValue);
								};
								
								addCloseOptionTask();
							})
						}
						else{
							region.splitDialogCache[regionAttr].msgDialogRegion.renderDialogData(queryResult,region.splitDialogCache[regionAttr].selectedItemArray);
							addCloseOptionTask();
						}
						
						function addCloseOptionTask(){
							setTimeout(function(){
								//清除事件
								var optionRegionRootObj = $(region.splitDialogCache[regionAttr].msgDialogRegion.getRegionDivElem());
								optionRegionRootObj.unbind("mouseout");
								optionRegionRootObj.find(".item").unbind("mouseover");
								
								optionRegionRootObj.mouseout(function(){
									optionRegionRootObj.attr("ov",0);
									if(shadowInput.closeOptionsTask!=null)clearTimeout(shadowInput.closeOptionsTask);
									shadowInput.closeOptionsTask = setTimeout(function(){//关闭窗口
										if(wrapped.searchTask!=null){
											clearTimeout(wrapped.searchTask);//避免延迟查询造成UI错位
										}
										if(region.splitDialogCache[regionAttr].msgDialogRegion!=null){
											RS.clearMsg(region.splitDialogCache[regionAttr].msgDialogRegion.regionId);
											region.splitDialogCache[regionAttr].msgDialogRegion = null;
										}
										wrapped.find(".shadow").blur();
									},200)
								})
								
								optionRegionRootObj.mouseover(function(){
									optionRegionRootObj.attr("ov",1);
									if(shadowInput.closeOptionsTask!=null)clearTimeout(shadowInput.closeOptionsTask);
								})
							})
						}
					});
				}
			}
			
			var newValue = JSON.stringify(region.splitDialogCache[regionAttr].selectedItemArray);
			wrapped.find(".wrapped").val(newValue);//TOTO
			
			//SplitInputPlugin.onchangeCallBack(wrapped.find(".wrapped"),newValue);
			
			//console.log(JSON.stringify(region.splitDialogCache[regionAttr].selectedItemArray))
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			var selectedItemArray = null;
			if(paraData!=null&&paraData!=""){
				try{
					selectedItemArray = JSON.parse(paraData);
				}
				catch(e){
					console.log("invalid data format");
					return;
				};
			}
			regionElemObj.val(paraData);
			
			var tmpItemHtml = "";
			if(selectedItemArray==null)selectedItemArray=[];
			for(var i = 0 ; i<selectedItemArray.length;i++){
				tmpItemHtml+='			<div class="item" value="'+selectedItemArray[i].value+'"><span class="item-text">'+selectedItemArray[i].text+'</span><i class="fa-solid fa-remove"></i></div>';
			}
			
			var regionAttr = regionElemObj.attr("region-attr");
			region.splitDialogCache[regionAttr].selectedItemArray = selectedItemArray;
			//console.log(wrapperElemObj.find(".items"))
			
			RS.setInnerHtml(region,wrapperElemObj.find(".items")[0],tmpItemHtml);
			
			setTimeout(function(){
				wrapperElemObj.find(".shadow").css("padding-left",wrapperElemObj.find(".items").width()+8);//设置左边宽度
			})
			
			wrapperElemObj.find(".item>.fa-remove").unbind("click");
			
			wrapperElemObj.find(".item>.fa-remove").click(function(){
				SplitInputPlugin.onRemoveClick(this);
			})
		},
		onRemoveClick:function(removeIcon){
			var curRegion = RS.getRegionByElem(removeIcon);
			
			var regionAttr = $(removeIcon).closest(".region-wrapper").children(".wrapped").attr("region-attr");
			var splitDialogCache = null;
			
			if(curRegion.type=="RegionGrid"){
				splitDialogCache = curRegion.regionForm.splitDialogCache[regionAttr];
			}
			else splitDialogCache = curRegion.splitDialogCache[regionAttr];
			
			var wrapped = splitDialogCache.wrapped;
			var uncheckVal = $(removeIcon).parent().attr("value");
			if(splitDialogCache.msgDialogRegion!=null)splitDialogCache.msgDialogRegion.uncheckValue(uncheckVal);//清除下拉里的选中状态
			$(removeIcon).parent().remove();
			var resultValArray = [];
			wrapped.find(".item").each(function(){
				var tmpObj = {};
				tmpObj.value = this.getAttribute("value");
				tmpObj.text = $(this).children(".item-text").text();
				resultValArray.push(tmpObj);
			})
			splitDialogCache.selectedItemArray = resultValArray;//重新赋值
			wrapped.find(".shadow").css("padding-left",wrapped.find(".items").width()+8);//.focus();
			
			var newValue = JSON.stringify(splitDialogCache.selectedItemArray);
			
			wrapped.find(".wrapped").val(newValue);
			SplitInputPlugin.onchangeCallBack(wrapped.find(".wrapped"),newValue);
			//console.log(JSON.stringify(splitDialogCache.selectedItemArray))
		},
		onchangeCallBack:function(wrappedObj,newValue){
			var onchange = wrappedObj.attr("onchange");
			if(onchange!=null)eval(onchange+'.call(wrappedObj[0],newValue)');
		}
}