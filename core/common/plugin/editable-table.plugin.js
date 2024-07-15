var EditableTablePlugin = {
		textHtml:"<input type=\"text\" region-attr=\"field\" class=\"inner text form-control\" maxlength=\"MAXLEN\" width=\"100%\">",
		sSelectHtml:"<select region-attr=\"field\" class=\"inner s-select form-control\" width=\"100%\" region-ds=\"DSName\"></select>",
		mSelectHtml:"<select type=\"multiple\" region-attr=\"field\" class=\"inner form-control m-select\" width=\"100%\" region-ds=\"DSName\"></select>",
		render:function(region,tmpObj,paraData){
			tmpObj.addClass("wrapped");
			tmpObj.addClass("hidden");
			var regionAttr = tmpObj.attr("region-attr");
			var showHeader = tmpObj.attr("show-header");
			if(showHeader==null||showHeader=="true"){
				showHeader = true;
			}
			else{
				showHeader = false;
			}
			
			var createable = tmpObj.attr("allow-create");//可新加
			
			var width = tmpObj.attr("width");
			
			if(width!=null)tmpObj.css("width",width);
			
			var height = tmpObj.attr("height");
			if(height==null)height="auto";
			
			var maxlength = tmpObj.attr("maxlength");
			var wrappedLabel = tmpObj.attr("label");//标签自带的label
			if(maxlength==null)maxlength=30;
			
			var replaceHtml = null;
			if(wrappedLabel==null){
				replaceHtml = '<div class="editable-table-wrapper region-wrapper">'+tmpObj.prop("outerHTML");
			}
			else{
				replaceHtml = '<div class="editable-table-wrapper region-wrapper"><span class="wrapped-label">'+wrappedLabel+'</span>'+tmpObj.prop("outerHTML");
			}
			
			var rowDataArray = null;
			if(paraData!=null&&paraData!=""){
				if (typeof paraData === 'string')
					rowDataArray = JSON.parse(paraData);
				else 
					rowDataArray = paraData;
			}
			
			
			
			var config,columns = null;
			if(region.configs==null){
				region.configs = {};
			}
			config = region.configs[regionAttr];
			if(config==null){
				region.configs[regionAttr]={};
			}
			
			//绘制表头
			if(config!=null)columns = config["columns"];
			
			if(columns==null){
				if(rowDataArray==null){
					rowDataArray = [//样本数据
						{
							"col1":"val1",
							"col2":"val2",
							"col3":"val3"
						},
						{
							"col1":"val3",
							"col2":"val4",
							"col3":"val5"
						}
					];
				}
				
				//没有表头数据则从表数据中创建表头
				//筛选出列字段最多的作为隐藏column划分
				for(var j = 0 ; j <rowDataArray.length ;j++){
					var rowData = rowDataArray[j];
					var newColumns = [];						
					var column = null;
					for(x in rowData){
						column = {};
						column["field"] = x;
						column["label"] = x;
						column["type"] = "text";
						column["width"] = "100px";//默认宽度
						newColumns.push(column);
					}
					if(columns==null||columns.length<newColumns)columns=newColumns;
				}
				region.configs[regionAttr]["columns"] = columns;//把自动创建的表头加入配置
			}
			
			if(columns==null)return;//数据错误
			
			if(showHeader)
				replaceHtml+='<div class="header-row">';
			else 
				replaceHtml+='<div class="header-row hidden">';
			for(var i = 0 ; i <columns.length ;i++){
				var columnConfig = columns[i];
				if(columnConfig["width"]==null){
					columnConfig["width"]="100px";//默认宽度
				}
				replaceHtml+='<div class="cell" style="width:'+columnConfig["width"]+'">'+columnConfig["label"]+'</div>';//row
			}
			replaceHtml+='<div class="cell" style="display: flex;justify-content: center;align-items: center;"><input class="promptCheck" type="checkbox"><span class="prompt">删除提示</span></div>';//控制列
			replaceHtml+='</div>';
			//绘制表头 结束

			//绘制数据
			replaceHtml+=EditableTablePlugin.resolveDataHtml(rowDataArray,columns,0);
			//绘制数据 结束
			
			if(createable==null||createable=="true")
				replaceHtml+='<div class="footer-row"><i class="newRowBtn fa-solid fa-plus"></i></div>';
			
			replaceHtml+='</div>';
			var replaced = $(replaceHtml);
			if(width!=null){
				replaced.css("width",width);
			}
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			var deletePropmt = tmpObj.attr("delete-propmt");
			if(deletePropmt==null||deletePropmt=="true")
				replaced.find(".promptCheck").prop("checked",true);
			
			//渲染下拉控件开始
			//DropdownPlugin.render(region,selectObj,paraData)
			replaced.find(".text").each(function(){
				var rowDataIndex = $(this).closest(".editable-row").attr("row-data-index");
				var rowData = rowDataArray[rowDataIndex];
				InputPlugin.render(region,$(this),rowData[$(this).attr("region-attr")]);
			});
			replaced.find(".s-select").each(function(){
				var rowDataIndex = $(this).closest(".editable-row").attr("row-data-index");
				var rowData = rowDataArray[rowDataIndex];
				DropdownPlugin.render(region,$(this),rowData[$(this).attr("region-attr")]);
			});
			replaced.find(".m-select").each(function(){
				var rowDataIndex = $(this).closest(".editable-row").attr("row-data-index");
				var rowData = rowDataArray[rowDataIndex];
				MultiDropdownPlugin.render(region,$(this),rowData[$(this).attr("region-attr")]);
			});
			//resolveInnerTags
			tmpObj = replaced.children(".wrapped");
			tmpObj.val(paraData);
			
			
			var dragable = tmpObj.attr("allow-drag");//可拖拽
			if(dragable==null||dragable=="true"){
				RegionUtil.enableChildDrag(replaced,".drag");
				replaced.find(".drag").removeClass("hidden");
			}

			var deleteable = tmpObj.attr("allow-delete");
			if(deleteable==null||deleteable=="true"){
				var removeBtns = replaced.find(".remove");
				removeBtns.removeClass("hidden");
				
				removeBtns.unbind("click");
				removeBtns.click(function(event){
					EditableTablePlugin.deleteRowData(event);
				});
			}
			
			if(createable==null||createable=="true"){
				var newBtn = replaced.find(".newRowBtn");
				newBtn.click(function(event){
					EditableTablePlugin.newRowData(event);
				});
			}
				
			
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			wrapperElemObj.find(".editable-row").remove();
			var rowDataArray = null;
			console.log(paraData)
			if(paraData!=null&&paraData!=""){
				if (typeof paraData == 'string')
					rowDataArray = JSON.parse(paraData);
				else 
					rowDataArray = paraData;
			}
			
			console.log("updateVal")
			console.log(rowDataArray)
			EditableTablePlugin.subUpdate(region,wrapperElemObj,regionElemObj,rowDataArray,0);
		},
		//添加一行数据
		newRowData:function(event){
			var targetBtn = RegionUtil.getEventTarget(event);
			var region = RegionUtil.getRegionByElem(targetBtn);
			
			var wrapperElemObj = $(targetBtn).closest(".editable-table-wrapper");
			var regionElemObj = wrapperElemObj.find(".wrapped");
			var rowCount = wrapperElemObj.find(".editable-row").length;
			EditableTablePlugin.subUpdate(region,wrapperElemObj,regionElemObj,[{}],rowCount);
			
			var maxRows = regionElemObj.attr("max-rows");
			if(maxRows!=null){
				try{
					maxRows = parseInt(maxRows);
				}
				catch(e){};
				if(maxRows!=null){
					if(maxRows==(rowCount+1)){//行数限制
						wrapperElemObj.find(".newRowBtn").addClass("hidden");
					}
				}
			}
		},
		deleteRowData:function(event){
			var targetBtn = RegionUtil.getEventTarget(event);
			var wrapperElemObj = $(targetBtn).closest(".editable-table-wrapper");
			var isChecked = wrapperElemObj.find(".promptCheck").prop("checked");
			var rowCount = wrapperElemObj.find(".editable-row").length;
			var regionElemObj = wrapperElemObj.find(".wrapped");
			var maxRows = regionElemObj.attr("max-rows");
			
			if(isChecked){
				RegionUtil.confirm("确认删除此行记录?","提示",function(){
					if(maxRows!=null){
						try{
							maxRows = parseInt(maxRows);
						}
						catch(e){};
						if(maxRows!=null){
							if(maxRows==rowCount){//行数限制
								wrapperElemObj.find(".newRowBtn").removeClass("hidden");
							}
						}
					}
					
					$(targetBtn).closest(".editable-row").remove();
				});
			}
			else{
				if(maxRows!=null){
					try{
						maxRows = parseInt(maxRows);
					}
					catch(e){};
					if(maxRows!=null){
						if(maxRows==rowCount){//行数限制
							wrapperElemObj.find(".newRowBtn").removeClass("hidden");
						}
					}
				}
				
				$(targetBtn).closest(".editable-row").remove();
			}
			
		},
		//根据传入的rowDataArray 添加UI行数据
		/**
		 * rowStartIndex 加入的行的行号
		 */
		subUpdate:function(region,wrapperElemObj,regionElemObj,rowDataArray,rowStartIndex){
			if(rowDataArray==null||rowDataArray.length==0)return;//空数据直接返回
			
			var regionAttr = regionElemObj.attr("region-attr");
			var columns = region.configs[regionAttr]["columns"];//列配置
			
			var dataHtml =EditableTablePlugin.resolveDataHtml(rowDataArray,columns,rowStartIndex);
			var footerRow = wrapperElemObj.find(".footer-row");
			var toInsert = $(dataHtml);
			if(footerRow.length==0){
				wrapperElemObj.append(toInsert);
			}
			else{
				toInsert.insertBefore(footerRow);
			}
			
			RegionUtil.addRegionUniqueId(toInsert[0],region.regionId);
			
			toInsert.find(".text").each(function(){
				var rowDataIndex = $(this).closest(".editable-row").attr("row-data-index");
				var rowData = rowDataArray[rowDataIndex-rowStartIndex];
				InputPlugin.render(region,$(this),rowData[$(this).attr("region-attr")]);
			});
			toInsert.find(".s-select").each(function(){
				var rowDataIndex = $(this).closest(".editable-row").attr("row-data-index");
				var rowData = rowDataArray[rowDataIndex-rowStartIndex];
				DropdownPlugin.render(region,$(this),rowData[$(this).attr("region-attr")]);
			});
			toInsert.find(".m-select").each(function(){
				var rowDataIndex = $(this).closest(".editable-row").attr("row-data-index");
				var rowData = rowDataArray[rowDataIndex-rowStartIndex];
				MultiDropdownPlugin.render(region,$(this),rowData[$(this).attr("region-attr")]);
			});	
			
			
			var dragable = regionElemObj.attr("allow-drag");//可拖拽
			if(dragable==null||dragable=="true"){
				RegionUtil.enableChildDrag(wrapperElemObj,".drag");
				toInsert.find(".drag").removeClass("hidden");
			}

			var deleteable = regionElemObj.attr("allow-delete");
			if(deleteable==null||deleteable=="true"){
				var removeBtns = toInsert.find(".remove");
				removeBtns.removeClass("hidden");
				
				removeBtns.unbind("click");
				removeBtns.click(function(event){
					EditableTablePlugin.deleteRowData(event);
				});
			}
		},
		
		//生成控件数据行的HTML
		resolveDataHtml:function(rowDataArray,columns,rowStartIndex){
			var dataRowHtml = "";
			if(rowDataArray!=null){
				for(var j = 0 ; j <rowDataArray.length ;j++){
					var rowData = rowDataArray[j];
					dataRowHtml+='<div class="editable-row" row-data-index="'+(rowStartIndex+j)+'">';
					
					for(var i = 0 ; i <columns.length ;i++){
						var columnConfig = columns[i];
						var textOverFlowStr = "";
						if(columnConfig["type"]=="text"){
							textOverFlowStr = "text-overflow";
						}
						
						dataRowHtml+='<div field="'+columnConfig["field"]+'"class="cell '+textOverFlowStr+'" style="width:'+columnConfig["width"]+'">';
						
						if(columnConfig["type"]=="text"){//普通文本
							var tmpTagHtml = EditableTablePlugin.textHtml.replace("field",columnConfig["field"]);
							var maxlength = columnConfig["maxlength"];
							if(maxlength==null)maxlength=30;
							tmpTagHtml=tmpTagHtml.replace("MAXLEN",maxlength);
							dataRowHtml+=tmpTagHtml;
						}
						else if(columnConfig["type"]=="s-select"){//下拉框
							var tmpTagHtml = EditableTablePlugin.sSelectHtml.replace("field",columnConfig["field"]);
							if(columnConfig["dataSourceType"]=="static"){
								var uniDsName = RegionUtil.getUUID()+"_DS";//产生随机DSname
								while(window.DataList.get(uniDsName)!=null){
									uniDsName = RegionUtil.getUUID()+"_DS";
								}
								
								window.DataList.put(uniDsName,columnConfig["dataSource"]);	
								tmpTagHtml = tmpTagHtml.replace("DSName",uniDsName);
							}
							else if(columnConfig["dataSourceType"]=="global"){
								tmpTagHtml = tmpTagHtml.replace("DSName",columnConfig["dataSource"]);
							}
								
							dataRowHtml += tmpTagHtml;
						}
						else if(columnConfig["type"]=="m-select"){//下拉框
							var tmpTagHtml = EditableTablePlugin.mSelectHtml.replace("field",columnConfig["field"]);
							if(columnConfig["dataSourceType"]=="static"){
								var uniDsName = RegionUtil.getUUID()+"_DS";//产生随机DSname
								while(window.DataList.get(uniDsName)!=null){
									uniDsName = RegionUtil.getUUID()+"_DS";
								}
								
								window.DataList.put(uniDsName,columnConfig["dataSource"]);	
								tmpTagHtml = tmpTagHtml.replace("DSName",uniDsName);
							}
							else if(columnConfig["dataSourceType"]=="global"){
								tmpTagHtml = tmpTagHtml.replace("DSName",columnConfig["dataSource"]);
							}
							
							dataRowHtml += tmpTagHtml;
						}
						dataRowHtml+='</div>';
					}
					dataRowHtml+='<div class="cell" style="width:80px"><i class="drag fa-solid fa-hand-rock hidden"></i><i class="remove fa-solid fa-trash hidden"></i></div>';
				
					dataRowHtml+='</div>';
				}
			}
			return dataRowHtml;
		},
		//控件数据打包
		packData:function(regionElemObj){
			var editableRows = regionElemObj.parent().find(".editable-row");
			var editableRow = null;
			var resultData = [];
			for(var i=0;i<editableRows.length;i++){
				editableRow = editableRows[i];
				var rowData = {};
				$(editableRow).children(".cell").each(function(index){
					var fieldName = this.getAttribute("field");
					if(fieldName==null)return true;
					rowData[fieldName] = $(this).find(".inner").val();
				});
				resultData.push(rowData);
			}
			if(resultData.length>0){
				return JSON.stringify(resultData);
			}
			return null;
		}
}












