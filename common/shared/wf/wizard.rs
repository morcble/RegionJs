<style type="text/css">
#REGION{
	 text-align: left;
	 position: relative;
}

#REGION .tabs{
	 display: inline-flex;
	 margin: 10px;
}

#REGION .forms{
	min-height: 400px;
}

#REGION .tabs>.stepItem.actived{
	background: var(--theme-waring);
}

#REGION .tabs>.stepItem.notAllowed{
	background: var(--theme-info);
	cursor: not-allowed;
}

#REGION .tabs>.stepItem{
	color:white;
    padding: 5px 40px;
    background: var(--theme-blue);
    margin: 0px 1px;
    cursor: pointer;
    min-width: 180px;
    text-align: center;
}

#REGION .tabs>.stepItem>.seqNo{
	padding-right: 10px;
	display: inline-block;
	font-size: 30px;
	height: 36px;
    vertical-align: middle;
}

#REGION .tabs>.stepItem>.title{
	display: inline-block;
	height: 36px;
	padding-top: 6px;
}

#REGION .controls>button{
	cursor: pointer;
}

#REGION .controls{
	padding: 20px;
}

#REGION .preview-controls{
	padding: 20px;
}

</style>

<div id="REGION" class="hidden">
	<div class="wizard">
		<div class="tabs"></div>
		
		<div class="forms"></div>
		
		<div class="controls text-center">
			<button class="btn btn-info btn-pre hidden" onclick="REGION.saveAndGoPre(event)"><message key="saveAndGoPre"></message></button>
			<button class="btn btn-waring btn-next hidden" onclick="REGION.saveAndGoNext(event)"><message key="saveAndGoNext"></message></button>
			<button class="btn btn-primary btn-save hidden" onclick="REGION.saveAsDraft(event)"><message key="saveAsDraft"></message></button>
			<button class="btn btn-primary btn-preview hidden" onclick="REGION.saveAndPreview(event)"><message key="preview"></message></button>
			<button class="btn btn-primary btn-savesubmit hidden" onclick="REGION.saveAndSubmit(event)"><message key="submit"></message></button>
		</div>
	</div>
	<div class="preview hidden">
		<div class="preview-content"></div>
		<div class="preview-controls text-center">
			<button class="btn"  data-type="primary" normal onclick="REGION.backFromPreview(event)"><message key="backFromPreview"></message></button>
			<button class="btn"  data-type="primary" normal onclick="REGION.submit(event)"><message key="submit"></message></button>
		</div>
	</div>
	
</div>	


<script type="text/javascript">
var REGION = {
		applicationId:null,
		afterRenderData:function(){
			this.renderWizard = REGION.renderWizard;
			REGION.applicationId = this.paraMap.get("applicationId");
			REGION.systemCode = this.paraMap.get("systemCode");
			REGION.workFlowName = this.paraMap.get("workFlowName");
		},
		submitApplication:function(curRegion){
			RegionUtil.confirm("确认提交申请?","提示",function(){
				var systemCode = SaasUtil.getSystemCode();
				var reqObj = {};
				reqObj.systemCode = REGION.systemCode;
				reqObj.workFlowName = REGION.workFlowName;
				reqObj.applicationId = REGION.applicationId;
				
				RegionUtil.loadingStart();
				var startWFPromise = RegionUtil.call(Config.backendPath+"/RCS_WorkFlow/workflow/submit-workflow","POST",reqObj);
				startWFPromise.done(function(serverResponse){
					if(serverResponse.success){
						if(typeof curRegion.onSubmitWorkflow ==="function"){
							let workflowId = serverResponse.data;
							let promise = curRegion.onSubmitWorkflow(workflowId,REGION.applicationId);
							if(promise!=null){
								promise.done(function(){
									RegionUtil.loadingComplete();
									RegionUtil.alert("申请提交成功");
									RegionUtil.closeModalWindow();
								})
								return;
							}
						}
						RegionUtil.alert("申请提交成功");
					}
					else{
						RegionUtil.alert(serverResponse.msg);
					}
					RegionUtil.loadingComplete();
					RegionUtil.closeModalWindow();					
				})
				
			})
		},
		submit:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			REGION.submitApplication(curRegion);
		},
		saveAndSubmit:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			if(typeof(curRegion.loadedSteps["stepForm"+curRegion.curStepIndex].saveStep) === "function"){
				var savePromise = REGION.saveAsDraft(event);
				if(savePromise!=null){
					savePromise.done(function(curStepRegion){
						REGION.submitApplication(curRegion);
					})
				}
			}
			else{
				REGION.submitApplication(curRegion);
			}
		},
		renderWizard:function(stepsData,stepIndex,previewRegion){//渲染步骤组件:申请主单ID，步骤数据，第几步,是否显示预览页面
			var curRegion = this;
			curRegion.stepsData = stepsData;
			curRegion.previewRegion = previewRegion;

			var htmlStr = "";
			if(stepsData.length>1){
				for(var i = 0 ; i <stepsData.length ;i++){
					htmlStr += '<div class="stepItem stepItem'+i+' notAllowed" onclick="javascript:REGION.saveAndJumpStep(event,'+stepsData[i].index+')">'
						+'<div class="seqNo">'+(stepsData[i].index+1)+'</div>'+'<div class="title">'+stepsData[i].title+'</div>'+'</div>';
				}
				RegionUtil.setInnerHtml(curRegion,curRegion.find(".tabs")[0],htmlStr);
			}
			else{
				curRegion.find(".tabs").addClass("hidden");
			}
			
			
			var rowTemplate = "<div class='stepForm hidden stepForm${index}'></div>";
			htmlStr = RegionUtil.wrapList(rowTemplate,stepsData);
			RegionUtil.setInnerHtml(curRegion,curRegion.find(".forms")[0],htmlStr);

			REGION.privateJumpStepInRegion(curRegion,stepIndex);
		},
		saveAndJumpStep:function(event,stepIndex){//event , 跳到哪个step上
			var stepItemDom = RegionUtil.getEventTarget(event);
			if($(stepItemDom).closest(".stepItem").hasClass("notAllowed"))return;
			
			var curRegion = RegionUtil.getRegionByEvent(event);
			
			if(curRegion.curStepIndex==stepIndex)return;
			
			REGION.privateSaveAndNavigate(curRegion,stepIndex,event);
		},
		saveAndGoPre:function(event){//往前跳
			var curRegion = RegionUtil.getRegionByEvent(event);
			REGION.privateSaveAndNavigate(curRegion,(curRegion.curStepIndex-1),event,true);
		},
		saveAndGoNext:function(event){//往后跳
			var curRegion = RegionUtil.getRegionByEvent(event);
			REGION.privateSaveAndNavigate(curRegion,(curRegion.curStepIndex+1),event);
		},
		saveAndPreview:function(event){//保存并预览
			var curRegion = RegionUtil.getRegionByEvent(event);
			if(typeof(curRegion.loadedSteps["stepForm"+curRegion.curStepIndex].saveStep) === "function"){
				var savePromise = REGION.saveAsDraft(event);
				if(savePromise!=null){
					savePromise.done(function(curStepRegion){
						//跳转到preview
						curRegion.find(".wizard").addClass("hidden");
						curRegion.find(".preview").removeClass("hidden");
						var paraMap = new HashMap();
						paraMap.put("applicationId",REGION.applicationId);
						var loadPromise = RegionUtil.loadRegion(curRegion.find(".preview-content"),RegionUtil.appendParaMapToUrl(curRegion.previewRegion,paraMap));
						loadPromise.done(function(loadedRegion){
							curRegion.previewRegionInstance = loadedRegion;
						});
					})
				}
			}
			else{
				//跳转到preview
				curRegion.find(".wizard").addClass("hidden");
				curRegion.find(".preview").removeClass("hidden");
				var paraMap = new HashMap();
				paraMap.put("applicationId",REGION.applicationId);
				var loadPromise = RegionUtil.loadRegion(curRegion.find(".preview-content"),RegionUtil.appendParaMapToUrl(curRegion.previewRegion,paraMap));
				loadPromise.done(function(loadedRegion){
					curRegion.previewRegionInstance = loadedRegion;
				});
			}
		},
		backFromPreview:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			curRegion.find(".preview").addClass("hidden");
			curRegion.find(".wizard").removeClass("hidden");
			curRegion.previewRegionInstance.release();
			curRegion.previewRegionInstance = null;
		},
		saveAndGoStep:function(curRegion,stepIndex,event){
			var curStepRegion = curRegion.loadedSteps["stepForm"+curRegion.curStepIndex];
			
			var savePromise = REGION.saveAsDraft(event);
			if(savePromise!=null){
				savePromise.done(function(curStepRegion){
					REGION.privateJumpStepInRegion(curRegion,stepIndex);
				})
			}
		},
		saveAsDraft:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			var curStepIndex = curRegion.curStepIndex;
			var curStepRegion = curRegion.loadedSteps["stepForm"+curStepIndex];

			var saveAsDraftPromise = null;
			
			var savePromise = curStepRegion.saveStep();
			if(savePromise!=null){
				var saveAsDraftDefer = new jQuery.Deferred();
				saveAsDraftPromise = saveAsDraftDefer.promise();
				
				savePromise.done(function(curStepRegion){
					if(curStepRegion.regionData!=null&&curStepRegion.regionData.id!=null){
						if(curStepIndex==0){
							REGION.applicationId = curStepRegion.regionData.id;
							curRegion.find(".stepItem").removeClass("notAllowed");
						}
						var toastPromise = RegionUtil.toast("保存成功",800);
						saveAsDraftDefer.resolve(curStepRegion);
					}
				})
			}
			return saveAsDraftPromise;
		},
		
		privateSaveAndNavigate:function(curRegion,stepIndex,event,needConfirm){
			if(typeof(curRegion.loadedSteps["stepForm"+curRegion.curStepIndex].saveStep) === "function"){
				if(needConfirm==true){
					RegionUtil.confirm("是否保存当前页面数据?","提示",function(){
						REGION.saveAndGoStep(curRegion,stepIndex,event);
					},function(){
						REGION.privateJumpStepInRegion(curRegion,stepIndex);//直接跳转
					})
				}
				else{
					REGION.saveAndGoStep(curRegion,stepIndex,event);
				}	
			}
			else{
				REGION.privateJumpStepInRegion(curRegion,stepIndex);//没有保存的接口就直接跳转
			}
		},
		privateJumpStepInRegion:function(curRegion,stepIndex){
			if(curRegion.curStepIndex==stepIndex)return;

			if(curRegion.loadedSteps == null){
				curRegion.loadedSteps = {};
			}

			curRegion.curStepIndex = stepIndex;
			
			var step = curRegion.stepsData[stepIndex];
			var stepId = "stepForm"+stepIndex;
			var stepLoadedRegion = curRegion.loadedSteps["stepForm"+stepIndex];
			
			
			var stepForm = curRegion.find(".stepForm"+stepIndex);
			stepForm.removeClass("hidden").siblings().addClass("hidden");//显示跳转的stepForm
			var stepItem = curRegion.find(".stepItem"+stepIndex);
			stepItem.addClass("actived").siblings().removeClass("actived");
			
			if(stepLoadedRegion==null||step.cache==false){
				var paraMap = new HashMap();
				if(REGION.applicationId!=null){
					paraMap.put(step.appIdAttrName,REGION.applicationId);//重要的传值
					paraMap.put("autoGet","true");//强制查询数据
				}
				var loadPromise = RegionUtil.loadRegion(stepForm,RegionUtil.appendParaMapToUrl(step.rsFile,paraMap));
				loadPromise.done(function(loadedRegion){
					loadedRegion.paraMap.remove("autoGet");//清除主动获取数据标记
					curRegion.curStepIndex = stepIndex;
					curRegion.loadedSteps["stepForm"+stepIndex] = loadedRegion;
					
					REGION.privateUpdateControls(curRegion);
					
					if(stepIndex==0){
						stepItem.removeClass("notAllowed");
						
						var applicationData = loadedRegion.regionData;//申请的主数据
						if(applicationData!=null && applicationData.id!=null){
							REGION.applicationId = applicationData.id;
							stepItem.siblings().removeClass("notAllowed");
						}
					}
					
				});
				return loadPromise;
			}
			else{
				curRegion.curStepIndex = stepIndex;
				REGION.privateUpdateControls(curRegion);
				
				//lazyimg被隐藏之后，重新显示需要修复
				window.setTimeout(function(){
					try{
						stepLoadedRegion.find('.lazyImg').lazyload({  
							placeholder : "images/loading.gif",
				        	threshold: 200,  
				        	effect: 'fadeIn',
				        	container: $(stepLoadedRegion.getRegionDivElem()),
				   	 	})
					}
					catch(e){
						
					}
					
				},10);
			}
			
			if(REGION.applicationId!=null){//所有step都可以点击
				curRegion.find(".stepItem").removeClass("notAllowed");
			}
		},
		privateUpdateControls:function(curRegion){//控制按钮显示
			var stepIndex = curRegion.curStepIndex;
			var controlsPanel = curRegion.find(".controls");
			controlsPanel.children().addClass("hidden");
			
			var totalStepCount = curRegion.stepsData.length;
			if(stepIndex>0)
				controlsPanel.find(".btn-pre").removeClass("hidden");
			
			if(stepIndex<(totalStepCount-1))
				controlsPanel.find(".btn-next").removeClass("hidden");
			
			if(stepIndex==(totalStepCount-1)){
				controlsPanel.find(".btn-savesubmit").removeClass("hidden");
				if(curRegion.previewRegion!=null)controlsPanel.find(".btn-preview").removeClass("hidden");
			}
			
			if(typeof(curRegion.loadedSteps["stepForm"+stepIndex].saveStep) === "function"){
				controlsPanel.find(".btn-save").removeClass("hidden");
			}
		}
};


RegionUtil.ready(function(){
	var res = {
			message:{
				dft:{
					saveAsDraft:"保存草稿",
					saveAndGoNext:"下一步",
					saveAndGoPre:"上一步",
					preview:"预览",
					submit:"提交申请",
					backFromPreview:"返回",
					
				}
			}/*,
			dataList:{
				dft:{
					sample:[{"text":"label1","value":"val1"},{"text":"label12","value":"val2"}]
				}
			}*/
	};
	
	var staticRegion = RegionUtil.newStaticRegion("#REGION",res);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>