/**
 * 动态表单
 */
var DynamicFormTemplates={		
		emptyStaticRegionHtml:"<div id=\"REGION\" class=\"hidden\">\n" + 
			"Sample Region\n" + 
			"</div>	\n" + 
			"<script type=\"text/javascript\">\n" + 
			"RegionUtil.ready(function(){\n" + 
			"	var staticRegion = RegionUtil.newStaticRegion(\"#REGION\");\n" + 
			"	staticRegion.renderRegion();\n" + 
			"})\n" + 
			"<\/script>",
		
		configuredRegionFormHtml: '<div id="REGION" class="hidden"></div>',
		configuredRegionFormScript: '<script type="text/javascript">\n'+	
			'		var REGION = {\n'+
			'@FUNC_HOLDER\n'+
			'		};\n'+
			'		RegionUtil.ready(function(){\n'+	
			'			var res = {\n'+
			'			    dataList:{\n'+
			'			    		dft:{\n'+
			'							@DS_HOLDER\n'+
			'						}\n'+
			'				}\n'+
			'			};\n'+
			'			var formRegion = RegionUtil.newFormRegion("#REGION",res);\n'+
			'			@VALIDATOR_HOLDER\n'+
			'			@CONFIG_HOLDER\n'+
			'			formRegion.renderRegion();\n'+	
			'			@INITATTR_HOLDER\n'+
		'})\n'+	
		'<\/script>',
		configuredRegionBtns: '<div class="data-block-ctrl"><span class="form-btn" onclick="REGION.validate(event)">验证表单</span><span class="form-btn" onclick="REGION.getFormData(event)">表单数据</span><span class="form-btn hidden" title="reset" onclick="REGION.reset(event)">重置</span></div>',
		
		'spliterHtml':'<div class="spliter"></div>',
		'sectionHtml':'<div class="section"><div class="section-content"></div></div>',
		
		
		'textHtml':'<input type="text" region-attr="property" class="form-control region-editable" maxlength="20" width="200px">',
		'm-textHtml':'<input type="text" region-attr="property" class="multi-text form-control region-editable" maxlength="20" width="400px" height="200px">',
		's-select':'<select region-attr="property" class="form-control region-editable" width="200px" region-ds="SampleOptions"></select>',
		'm-selectHtml':'<select type="multiple" region-attr="property" class="form-control region-editable" width="200px" region-ds="SampleOptions"></select>',
		
		's-chooseHtml':'<input type="radio" region-attr="property" class="form-control region-editable" region-ds="SampleOptions">',
		'm-chooseHtml':'<input type="checkbox" region-attr="property" class="form-control region-editable" region-ds="SampleOptions">',
		
		'dateHtml':'<input type="text" region-attr="property" class="date form-control region-editable" width="200px" format="YYYY-MM-DD" locale="cn">',
		'starHtml':'<input type="text" region-attr="property" class="star form-control region-editable" max-score="5">',
		'switchHtml':'<input type="text" region-attr="property" class="switch form-control region-editable" active-text="开"  inactive-text="关">',
		'richinputHtml':'<input type="text" region-attr="property" class="richinput form-control region-editable" width="80%">',
		
		'fileHtml':'<input type="file" region-attr="property"  class="region-editable"  paras="maxsize=2000;accept=*" vendor="local" download-able="true">',
		'imgHtml':'<input region-attr="property" class="img region-editable" paras="maxsize=2000;accept=*" vendor="local" download-able="true">',
		
		'transferHtml':'<input type="text" class="transfer region-editable" region-attr="property" source-title="待选项"  result-title="被选项" width="500px" height="200px">',
		'edittableHtml':'<input type="text" region-attr="property" class="editable-table form-control region-editable" max-rows="5" width="auto">',
		'colorpickerHtml':'<input type="text" class="colorpicker region-editable" region-attr="property">',
		'numcounterHtml':'<input type="text" class="numcounter region-editable" region-attr="property" step="1">',
		'sliderHtml':'<input type="text" class="slider region-editable" region-attr="property" width="200px" min="0" max="10">',
		'timepickerHtml':'<input type="text" class="timepicker form-control region-editable" region-attr="property">',
		
		
		'view-textHtml':'<span class="view-text region-editable" region-attr="property"></span>',
		'view-dsHtml':'<span class="view-ds region-editable" region-attr="property" region-ds="SampleOptions"></span>',
		'view-barcodeHtml':'<view class="barcode region-editable" region-attr="property" height="50px"></view>',
		'view-qrcodeHtml':'<view class="qrcode region-editable" region-attr="property"></view>',
		'view-codeHtml':'<view class="code region-editable" region-attr="property" width="100%" name="BookService.java"></view>',
		
		'editorHtml':'<region resType="relativeUrl" res="tools/formeditor/form-editor-index.rs"></region>',
		'regionTreeHtml':'<region resType="relativeUrl" res="samples/examples/tree/simple-tree-example.rs"></region>',
		'regionSelectableTreeHtml':'<region resType="relativeUrl" res="samples/examples/tree/checkbox-tree-example.rs"></region>',
		'regionMenuHtml':'<region resType="relativeUrl" res="samples/examples/stackmenu/stackmenu-example.rs"></region>',
		'regionHtml':'<region resType="html" region-attr="property" class="region-editable"></region>',
	
		
		'blockHtmlStartPart_Editable':'<div class="data-block-wrapper editable" style="width:blockWidth;" draggable="false">' + 
				'			<div class="data-block titlePosition readonly" style="width:100%;">' + 
				'			    <div class="field-label">' + 
				'			    <label>LABEL</label>' + 
				'			    </div>' + 
				'			    <div class="field-val">' 
		,
		'blockHtmlEndPart_Editable':'		</div>' + 
				'			</div>' + 
				'			<div class="block-control hidden"><i class="fa fa-copy"></i><i class="fa-solid fa-trash-alt"></i></div>' + 
				'		</div>'
		,
		'plainBlockHtmlStartPart_Editable':'<div class="data-block-wrapper editable" style="width:blockWidth;" draggable="false">' + 
				'			<div class="data-block readonly" style="width:100%;">'
		,
		'plainBlockHtmlEndPart_Editable':'			</div>' + 
				'			<div class="block-control hidden"><i class="fa fa-copy"></i><i class="fa-solid fa-trash-alt"></i></div>' + 
				'		</div>'
		,
		'blockHtmlStartPart':'<div class="data-block-wrapper" style="width:blockWidth;">' + 
				'			<div class="data-block titlePosition" style="width:100%;">' + 
				'			    <div class="field-label">' + 
				'			    	 <label>LABEL</label>' + 
				'			    </div>' + 
				'			    <div class="field-val">' 
		,
		'blockHtmlEndPart':'		</div>' + 
				'			</div>' + 
				'		</div>'
		,
		'plainBlockHtmlStartPart':'<div class="data-block-wrapper" style="width:blockWidth;">' + 
				'			<div class="data-block" style="width:100%;">'
		,
		'plainBlockHtmlEndPart':'			</div>' + 
				'		</div>'
		,
		getHtmlForItem:function(editMode,itemType,subType,itemTitle,titlePosition,blockWidth,fieldName){//解析form html
			var startAndEndPartSuffix = "";
			if(editMode==true){
				startAndEndPartSuffix = "_Editable";
			}
			
			var appendHtml = null;
			if(itemType=="spliter"){//栅格特殊处理
				appendHtml = DynamicFormTemplates["plainBlockHtmlStartPart"+startAndEndPartSuffix] + DynamicFormTemplates.spliterHtml 
							+ DynamicFormTemplates["plainBlockHtmlEndPart"+startAndEndPartSuffix];
				appendHtml=appendHtml.replace('blockWidth',"100%");
				
				appendHtml=appendHtml.replace('data-block-wrapper',"data-block-wrapper horizontal-line");
			}
			else if(itemType=="section"){//栅格特殊处理
				appendHtml = DynamicFormTemplates["plainBlockHtmlStartPart"+startAndEndPartSuffix] + DynamicFormTemplates.sectionHtml 
							+ DynamicFormTemplates["plainBlockHtmlEndPart"+startAndEndPartSuffix];
				appendHtml=appendHtml.replace('blockWidth',"100%");
			}
			else if(itemType=="region"){
				appendHtml = DynamicFormTemplates["plainBlockHtmlStartPart"+startAndEndPartSuffix];
				if(subType=="tree"){
					appendHtml+= DynamicFormTemplates.regionTreeHtml;
				}
				else if(subType=="selectable-tree"){
					appendHtml+= DynamicFormTemplates.regionSelectableTreeHtml;
				}
				else if(subType=="menu"){
					appendHtml+= DynamicFormTemplates.regionMenuHtml;
				}
				else if(subType=="editor"){
					appendHtml+= DynamicFormTemplates.editorHtml;
				}
				else{
					appendHtml+= DynamicFormTemplates.regionHtml;
				}
				appendHtml+=DynamicFormTemplates["plainBlockHtmlEndPart"+startAndEndPartSuffix].replace("fa-copy","fa-copy hidden");//禁止组件拷贝
				
	
			}
			else{
				appendHtml = DynamicFormTemplates["blockHtmlStartPart"+startAndEndPartSuffix];
				if(titlePosition==0){
					appendHtml=appendHtml.replace('titlePosition','right-title');
				}
				else if(titlePosition==1){
					appendHtml=appendHtml.replace('titlePosition','left-title');
				}
				else if(titlePosition==2){
					appendHtml=appendHtml.replace('titlePosition','top-title');
				}
				
				appendHtml=appendHtml.replace('blockWidth',blockWidth);
				if(itemType=="text"){
					appendHtml+= DynamicFormTemplates.textHtml;
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="m-text"){
					appendHtml+= DynamicFormTemplates["m-textHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="s-select"){
					appendHtml+= DynamicFormTemplates["s-select"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="m-select"){
					appendHtml+= DynamicFormTemplates["m-selectHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="s-choose"){
					appendHtml+= DynamicFormTemplates["s-chooseHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="m-choose"){
					appendHtml+= DynamicFormTemplates["m-chooseHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="date"){
					appendHtml+= DynamicFormTemplates["dateHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="star"){
					appendHtml+= DynamicFormTemplates["starHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="switch"){
					appendHtml+= DynamicFormTemplates["switchHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="richinput"){
					appendHtml+= DynamicFormTemplates["richinputHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="file"){
					appendHtml+= DynamicFormTemplates["fileHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="img"){
					appendHtml+= DynamicFormTemplates["imgHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="colorpicker"){
					appendHtml+= DynamicFormTemplates["colorpickerHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="transfer"){
					appendHtml+= DynamicFormTemplates["transferHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="numcounter"){
					appendHtml+= DynamicFormTemplates["numcounterHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="slider"){
					appendHtml+= DynamicFormTemplates["sliderHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="timepicker"){
					appendHtml+= DynamicFormTemplates["timepickerHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="edittable"){
					appendHtml+= DynamicFormTemplates["edittableHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="view-text"){
					appendHtml+= DynamicFormTemplates["view-textHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="view-ds"){
					appendHtml+= DynamicFormTemplates["view-dsHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="view-barcode"){
					appendHtml+= DynamicFormTemplates["view-barcodeHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="view-qrcode"){
					appendHtml+= DynamicFormTemplates["view-qrcodeHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
				else if(itemType=="view-code"){
					appendHtml+= DynamicFormTemplates["view-codeHtml"];
					appendHtml = appendHtml.replace('LABEL',itemTitle);
				}
			}
			

			appendHtml = appendHtml.replace('property',fieldName);
			appendHtml+= DynamicFormTemplates["blockHtmlEndPart"+startAndEndPartSuffix];	
			return appendHtml;
		},
		resolveListRegionContent:function(configData,useFormBtns,bindWithDatasrc,datasrcName){//解析html内容
			var tagItemConfigs = configData.tagItemConfigs;
			var itemsHtml = "";
			var appendDsMap = new HashMap();//datals属性
			var validatorScriptArray = [];//validator属性

			var appendDomObject = null;
			var appendHtml = '<div id="REGION" class="hidden">';
			for(var i = 0 ; i<tagItemConfigs.length ;i++){
				appendHtml+='	<input type="hidden" region-attr="'+tagItemConfigs[i].fieldName+'" class="form-control region-searchable" maxlength="20">';
			}
			
			appendHtml+='		<div class="form">';
			appendHtml+='			<div class="form-info">';
			appendHtml+='				<div class="title">列表</div>';
			appendHtml+='				<div class="deleteopt"><button class="btn btn-info batch-delete hidden"><message key="global_msg.batch_delete"></message></button></div>';
			appendHtml+='				<div class="toolbar">';
			//appendHtml+='					<i class="fa fa-search grid-search" title="search"></i>';
			appendHtml+='					<i class="fa fa-plus grid-plus" title="add"></i>';
			appendHtml+='					<i class="fa fa-refresh region-refresh" title="refresh"></i>';
			appendHtml+='				</div>';
			appendHtml+='			</div>';
			appendHtml+='		<div class="form-body">';
			appendHtml+='			<div class="datas-columns header-row">';
			appendHtml+='				<div class="col-xs-1 maxwidth40"><input class="selectAll" type="checkbox"></div>';
			appendHtml+='				<div class="col-xs-1 maxwidth60"><message key="global_msg.index"></message></div>';
			for(var i = 0 ; i<tagItemConfigs.length ;i++){
				var title = tagItemConfigs[i].title;
				if(title.endWith(":"))title=title.substring(0,title.length-1);
				appendHtml+='				<div class="col-xs-1"><label>'+title+'</label></div>';
			}
			
			appendHtml+='				<div class="col-xs-2"><message key="global_msg.operation"></message></div>';
			appendHtml+='			</div>';
			appendHtml+='			<div class="datas" region-list="list">';
			appendHtml+='				<div class="row" onclick="RegionUtil.handleListData(event,REGION.onRowclick)">';
			appendHtml+='					<div class="col-xs-1 maxwidth40"><input type="checkbox" class="multipleCbs"></div>';
			appendHtml+='					<div class="col-xs-1 maxwidth60"><span region-attr="index"></span></div>';
			for(var i = 0 ; i<tagItemConfigs.length ;i++){
				if(tagItemConfigs[i].itemType=="s-select"||tagItemConfigs[i].itemType=="m-select"||tagItemConfigs[i].itemType=="s-choose"||tagItemConfigs[i].itemType=="m-choose"){
					if(tagItemConfigs[i].dataSrcType=="static"){
						var dataSrcRandomId = "ds_"+tagItemConfigs[i].fieldName;
						appendDsMap.put(dataSrcRandomId,JSON.stringify(tagItemConfigs[i].staticDataSrc));
						appendHtml+='				<div class="col-xs-1"><span region-ds="'+dataSrcRandomId+'" region-attr="'+tagItemConfigs[i].fieldName+'"></span></div>';
						continue;
					}
				}
				appendHtml+='				<div class="col-xs-1"><span region-attr="'+tagItemConfigs[i].fieldName+'"></span></div>';
			}
			
			appendHtml+='					<div class="col-xs-2">';
			appendHtml+='						<a href="javascript:void(0)" onclick="RegionUtil.handleListData(event,REGION.viewData)"><i class="fa fa-pencil fa-fw" title="编辑"></i></a>';
			appendHtml+='					<button class="btn" data-type="danger" plain onclick="RegionUtil.handleListData(event,REGION.deleteData)"><i class="fa-light fa-trash-alt" title="Delete"></i></button>';
			appendHtml+='					</div>'; 
			appendHtml+='				</div>'; 
			appendHtml+='			</div>';
	
			appendHtml+='			<div class="paginationControl"></div>';
			appendHtml+='			<div class="norecordmsg fa hidden"><message key="global_msg.no_record_found"></message></div>';
			appendHtml+='		</div>';//end of form-body
			appendHtml+='	</div>';
			appendHtml+='</div>';
			
	
			var finalFormContent = DynamicFormTemplates.resolveRegionContent(configData,useFormBtns,bindWithDatasrc,datasrcName);
			var formUrl = RegionUtil.getRegionCachedUrlByContent(finalFormContent);
			
			appendHtml+='<script type="text/javascript">';
			
			
			appendHtml+='var REGION = {';
			appendHtml+='		onRowclick:function(rowData){';
			appendHtml+='			if(this.rowClick!=null)this.rowClick(rowData);';
			appendHtml+='		},';
			appendHtml+='		viewData :function (rowData){';
			appendHtml+='			this.viewRecord(rowData,this.regionId);';
			appendHtml+='		},';
			appendHtml+='		deleteData:function(rowData){';
			appendHtml+='			this.deleteData(rowData);';
			appendHtml+='		}';
			appendHtml+='};';
			
			appendHtml+='	var res = {';
			appendHtml+='		dataList:{';
			appendHtml += '			dft:{';
			
			if(appendDsMap.size()!=0){
				var keys = appendDsMap.keySet();
				var dsScript = "";
				for(var i=0;i<keys.length;i++){
					dsScript+=keys[i]+":"+appendDsMap.get(keys[i])+",\n";
				}
				appendHtml +=dsScript;
			}
			appendHtml += '			}';
			appendHtml += '		}';
			appendHtml += '	};';
			appendHtml += '	var gridRegion = RegionUtil.newGridRegion("#REGION",res);';
			appendHtml += '	gridRegion.resUrl = Config.backendPath+ "/subproject/cloud/form/'+datasrcName+'/list";	';
			appendHtml += '	gridRegion.addTitle = global_msg._new+"'+datasrcName+'";';
			appendHtml += '	gridRegion.viewTitle = global_msg._edit+"'+datasrcName+'";';
			appendHtml += '	gridRegion.reqRes = "'+formUrl+'";';
			
			appendHtml += '	gridRegion.setRowClick = function(functionStrPara,para){';
			appendHtml += '		var functionStr = "gridRegion.rowClick = function("+para+"){"+functionStrPara+"}";';
			appendHtml += '		eval(functionStr);';
			appendHtml += '	};';
			
			appendHtml += '	gridRegion.search();	';
			appendHtml += '</script>';
			//
			
			
			if(Config.REGION_DEBUG_MODE)console.log(appendHtml);
			return appendHtml;
		},
		resolveRegionContent:function(configData,useFormBtns,bindWithDatasrc,datasrcName){
			if(typeof(configData)=="string"){
				configData = JSON.parse(configData);
			}
			
			var tagItemConfigs = configData.tagItemConfigs;

			var globalConfig = configData.globalConfig;
			if(globalConfig==null){
				globalConfig = {};
			}
			var blockParentClass = null;

			if(globalConfig.titlePosition=="1")
				blockParentClass = "left-title";
			else if(globalConfig.titlePosition=="2")
				blockParentClass = "top-title";
			else blockParentClass = "right-title";
			
			if(globalConfig.blockWidth == "50%")
				blockParentClass+=" two-column";
			else if(globalConfig.blockWidth == "33.3%")
				blockParentClass+=" three-column";
			else if(globalConfig.blockWidth == "25%")
				blockParentClass+=" four-column";
			else
				blockParentClass+=" one-column";
			
			
			var itemsHtml = "";
			var initAttrScriptsMap = new HashMap();//初始化属性
			var configScriptsMap = new HashMap();//配置属性
			var appendDsMap = new HashMap();//datals属性
			var validatorScriptArray = [];//validator属性
			
			
			if(bindWithDatasrc){
				itemsHtml = '<input region-attr="id" class="region-editable" type="hidden">';
				itemsHtml+='<div class="form">';
				itemsHtml+='	<div class="form-info">';
				itemsHtml+='		<div class="title">'+datasrcName+'详细</div>';
				itemsHtml+='		<div class="toolbar">';
				itemsHtml+='			<i class="fa fa fa-save" title="save" onclick="REGION.saveRegion(event)"></i>';
				itemsHtml+='			<i class="fa fa fa-undo" title="reset" onclick="REGION.reset(event)"></i>';
				itemsHtml+='			<i class="fa fa-refresh region-refresh" title="refresh" onclick="REGION.refreshRegion(event)"></i>';
				itemsHtml+='		</div>';
				itemsHtml+='	</div>';
				itemsHtml+='	<div class="form-body '+blockParentClass+'">';
			}
			else{
				itemsHtml='<div class="match-parent '+blockParentClass+'">';
			}
			
			
			
			for(var i = 0 ; i<tagItemConfigs.length ;i++){
				var tagItemConfig = tagItemConfigs[i];
				var jsonFormatAttrs = tagItemConfig.jsonFormatAttrs;
				if(jsonFormatAttrs!=null&& jsonFormatAttrs.length>0){
					for(var j = 0 ; j <jsonFormatAttrs.length ; j++){
						var origAttr = jsonFormatAttrs[j];
						tagItemConfig[origAttr] = JSON.stringify(tagItemConfig[origAttr]);
					}
				} 
				itemsHtml+=DynamicFormTemplates.handleTagItemConfig(tagItemConfig,initAttrScriptsMap,appendDsMap,configScriptsMap,validatorScriptArray);
			}
			
			if(bindWithDatasrc){
				itemsHtml+="	</div>";
				itemsHtml+="</div>";
			}
			else{
				itemsHtml+="</div>";
			}
			
			var regionHtml = DynamicFormTemplates.configuredRegionFormHtml;
			var regionObj = $(regionHtml);
			
			var scriptPart = DynamicFormTemplates.configuredRegionFormScript;
			if(bindWithDatasrc){
				var replaceStr = 'formRegion.resUrl = Config.backendPath+"/subproject/cloud/form/'+datasrcName+'/";formRegion.renderRegion();formRegion.saveSuccessCallBack = REGION.saveSuccessCallBack;formRegion.saveFailedCallBack = REGION.saveFailedCallBack;';
				scriptPart = scriptPart.replace("formRegion.renderRegion();",replaceStr);
			}
			
			
			
			//---------------------------------------------------------
			if(appendDsMap.size()==0){
				scriptPart = scriptPart.replace("@DS_HOLDER","");
			}
			else{
				var keys = appendDsMap.keySet();
				var dsScript = "";
				for(var i=0;i<keys.length;i++){
					dsScript+=keys[i]+":"+appendDsMap.get(keys[i])+",\n";
				}
				scriptPart = scriptPart.replace("@DS_HOLDER",dsScript);
			}
			//---------------------------------------------------------
			if(configScriptsMap.size()==0){
				scriptPart = scriptPart.replace("@CONFIG_HOLDER","");
			}
			else{
				var keys = configScriptsMap.keySet();
				var configScript = "";
				for(var i=0;i<keys.length;i++){
					configScript+='			formRegion.config("'+keys[i]+'",\n';
					configScript+='				'+configScriptsMap.get(keys[i])+'\n';
					configScript+='			);\n';
				}
				scriptPart = scriptPart.replace("@CONFIG_HOLDER",configScript);
			}
			//---------------------------------------------------------
			if(initAttrScriptsMap.size()==0){
				scriptPart = scriptPart.replace("@INITATTR_HOLDER","");
			}
			else{
				var keys = initAttrScriptsMap.keySet();
				var initAttrScript = "";
				for(var i=0;i<keys.length;i++){
					initAttrScript+="\n			formRegion.setAttr('"+keys[i]+"','"+initAttrScriptsMap.get(keys[i])+"');";
				}
				scriptPart = scriptPart.replace("@INITATTR_HOLDER",initAttrScript);
			}
			
			//---------------------------------------------------------
			if(validatorScriptArray.length==0){
				scriptPart = scriptPart.replace("@VALIDATOR_HOLDER","");
			}
			else{
				var validatorScript = "";
				for(var i=0;i<validatorScriptArray.length;i++){
					validatorScript+=validatorScriptArray[i];
				}
				validatorScript+='\n'
				scriptPart = scriptPart.replace("@VALIDATOR_HOLDER",validatorScript);
			}
			//----------控制按钮脚本-----------------------------------------------
			var regionFunctions="";
			regionFunctions+="validate:function(event){\n";
			regionFunctions+="	RegionUtil.getRegionByEvent(event).validate();\n";
			regionFunctions+="},\n";
			regionFunctions+="reset:function(event){\n";
			regionFunctions+="RegionUtil.getRegionByEvent(event).reset();\n";
			regionFunctions+="},\n";
			regionFunctions+="getFormData:function(event){\n";
			regionFunctions+="var formData = RegionUtil.getRegionByEvent(event).packFormData();\n";
			regionFunctions+="var paraMap = new HashMap();\n";
			regionFunctions+='paraMap.put("result",formData);\n';
			regionFunctions+='RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("tools/formeditor/modules/export-json-view.rs",paraMap),"表单数据",1,0.5,0.7,600);\n';
			regionFunctions+="},\n";
			
			
			regionFunctions+='saveRegion:function(event){';
			regionFunctions+='	var tmpRegion = RegionUtil.getRegionByEvent(event);';
			regionFunctions+='	tmpRegion.saveOrUpdateRegion();';
			regionFunctions+='},';
			regionFunctions+='refreshRegion:function(event){';
			regionFunctions+='	var tmpRegion = RegionUtil.getRegionByEvent(event);';
			regionFunctions+='	tmpRegion.refreshRegion();';
			regionFunctions+='},';
			regionFunctions+='saveSuccessCallBack:function(){';
			regionFunctions+='	if(parent!=null){';
			regionFunctions+='		try{';
			regionFunctions+='			RegionUtil.alert(global_msg.save_successfully);';
			regionFunctions+='			if(this.isPopup()){';
			regionFunctions+='				RegionUtil.getWindowRegionById(parent.window,this.paraMap.get("refreshRegionId")).refreshRegion();';
			regionFunctions+='				parent.closeModalWindow();';
			regionFunctions+='			}';
			regionFunctions+='		}catch(e){';
			regionFunctions+='			console.log(e)';
			regionFunctions+='		}';
			regionFunctions+='	}';
			regionFunctions+='},';
			regionFunctions+='saveFailedCallBack:function(){';
			regionFunctions+='	console.log("save failed");';
			regionFunctions+='}';
			
		
			
			scriptPart = scriptPart.replace("@FUNC_HOLDER",regionFunctions);
			//---------------------------------------------------------
			$(regionObj.append(itemsHtml).append(DynamicFormTemplates.configuredRegionBtns));
			if(useFormBtns!=true){
				regionObj.find(".data-block-ctrl").addClass("hidden");
			}
			
			
			var finalFormContent = regionObj.prop("outerHTML")+scriptPart;
			//console.log(finalFormContent);
			return finalFormContent;
		},
		handleTagItemConfig:function(tagItemConfig,initAttrScriptsMap,appendDsMap,configScriptsMap,validatorScriptArray){
			var itemType = tagItemConfig.itemType;
			//console.log(tagItemConfig)
			var appendDomObject = null;
			var appendHtml = DynamicFormTemplates.getHtmlForItem(false,tagItemConfig.itemType,tagItemConfig.subType,tagItemConfig.title,tagItemConfig.titlePosition,tagItemConfig.blockWidth,tagItemConfig.fieldName);
			if(tagItemConfig.itemType=="s-select"||tagItemConfig.itemType=="m-select"||tagItemConfig.itemType=="s-choose"||tagItemConfig.itemType=="m-choose"){
				var dataSrcType = tagItemConfig.dataSrcType;
				if(dataSrcType==null){
					//TODO check
				}
				else if(dataSrcType=="static"){
					if(tagItemConfig.staticDataSrc==null){
						//TODO check
					}
					var dataSrcRandomId = "ds_"+tagItemConfig.fieldName;
					appendDsMap.put(dataSrcRandomId,tagItemConfig.staticDataSrc);
					appendHtml = appendHtml.replace('SampleOptions',dataSrcRandomId);
				}
				else{
					appendHtml = appendHtml.replace('SampleOptions',tagItemConfig.formDataSrc);
				}
			}
			else if (tagItemConfig.itemType=="edittable"){
				if(tagItemConfig.columns!=null){
					configScriptsMap.put(tagItemConfig.fieldName,JSON.stringify({"columns":JSON.parse(tagItemConfig.columns)}));
				}
			}
			else if (tagItemConfig.itemType=="date"){
				if(tagItemConfig.specialDays!=null){
					configScriptsMap.put(tagItemConfig.fieldName,JSON.stringify({"specialDays":JSON.parse(tagItemConfig.specialDays)}));
				}
			}
			else if (tagItemConfig.itemType=="section"){
				//特殊处理
				console.log(tagItemConfig)
				var appendDomObject = $(appendHtml);
				var sectionContent = appendDomObject.find(".section-content");
				if(tagItemConfig.text!=null)sectionContent.text(tagItemConfig.text);
				sectionContent.css("line-height",tagItemConfig.lineheight);
				sectionContent.css("font-size",tagItemConfig.fontsize);
				sectionContent.css("font-weight",tagItemConfig.fontweight);
				
				appendDomObject.find(".section").css("background",tagItemConfig.bgcolor);

				return appendDomObject.prop("outerHTML")+'\n';
			}
			else if (tagItemConfig.itemType=="region"){
				appendDomObject = $(appendHtml);
				var regionTag = appendDomObject.find("REGION");
				regionTag.attr("resType",tagItemConfig.resType);
				regionTag.attr("res",tagItemConfig.res);
				regionTag.attr("region-uuid",tagItemConfig.uuid);
				regionTag.attr("region-attr",tagItemConfig.fieldName);
				regionTag.attr("width",tagItemConfig.width);
				regionTag.attr("height",tagItemConfig.height);
			}

			
			if(tagItemConfig["initData"]!=null){
				initAttrScriptsMap.put(tagItemConfig.fieldName,tagItemConfig["initData"]);
			}
			
			//处理数据验证 validateMsg  validationType  mandatory TODO统一处理
			var validationRegArray = [];
			var validationMsgArray = [];
			if(tagItemConfig.mandatory=="true"){
				validationRegArray.push(SysValidationTypes["100"].regx);
				validationMsgArray.push(SysValidationTypes["100"].msg);
			}
			

			if(tagItemConfig.validationType!=null && tagItemConfig.validationType!=0){
				validationRegArray.push(SysValidationTypes[tagItemConfig.validationType].regx);
				validationMsgArray.push(SysValidationTypes[tagItemConfig.validationType].msg);
			}
			
	
			if(validationRegArray.length!=0){
				var validateScript='\n			formRegion.addValidator("'+tagItemConfig.fieldName+'",new Array(';
				for(var i = 0 ;i<validationRegArray.length;i++){
					validateScript+= validationRegArray[i]+",";	
				}
				validateScript = validateScript.substring(0,validateScript.length-1);
				
				validateScript+='),new Array(';
				
				for(var i = 0 ;i<validationMsgArray.length;i++){
					validateScript+= "'"+validationMsgArray[i]+"',";
				}
				validateScript = validateScript.substring(0,validateScript.length-1);
				
				validateScript+="));"
				validatorScriptArray.push(validateScript);
			}
			if(appendDomObject==null)appendDomObject = $(appendHtml);
			
			if (tagItemConfig.itemType=="img"){
				var paras="maxsize="+tagItemConfig.maxsize+";accept="+tagItemConfig.accept;
				appendDomObject.find(".img").attr("paras",paras);
				
				delete tagItemConfig.maxsize;
				delete tagItemConfig.accept;
			}
			else if (tagItemConfig.itemType=="file"){
				var paras="maxsize="+tagItemConfig.maxsize+";accept="+tagItemConfig.accept;
				appendDomObject.find("input[type=file]").attr("paras",paras);
				
				delete tagItemConfig.maxsize;
				delete tagItemConfig.accept;
			}
			
			delete tagItemConfig.fieldName;
			delete tagItemConfig.itemType;
			delete tagItemConfig.subType;
			delete tagItemConfig.titlePosition;
			delete tagItemConfig.blockWidth;
			delete tagItemConfig.jsonFormatAttrs;
			
			//datasource
			delete tagItemConfig.datasrctype;
			delete tagItemConfig.formdatasrc;
			delete tagItemConfig.staticDataSrc;
			
			//可编辑表格
			delete tagItemConfig.columns;
			
			//日期
			delete tagItemConfig.specialDays;
			
			//初始化数据
			delete tagItemConfig.initData;

			var tagElem = appendDomObject.find(".region-editable");
			for(x in tagItemConfig){
				if(tagItemConfig[x]!=null){
					tagElem.attr(x,tagItemConfig[x]);
				}
			}
			return appendDomObject.prop("outerHTML")+'\n';
		}
}



