<style type="text/css">
#REGION{
	padding-bottom: 6rem;
	position: relative;
}
#REGION>.steps{
	width: 100%;
	height: 100%;
	overflow: auto;
}

#REGION>.control{
	position: absolute;
	bottom: 0rem;
	right: 2rem;
	height: 5rem;
	width: 100%;
}

#REGION>.control button{
	margin-left: 2rem;
}

</style>
<!-- 
控制的step组件需根据情况实现 pre  , next 函数
 -->
<div id="REGION" class="hidden">
	<div class="steps"></div>
	<div class="control">
		<div class="text-center">
			<button class="btn hidden pre" data-type="default" normal onclick="REGION.pre(event)">-</button>
			<button class="btn next" data-type="primary" normal onclick="REGION.next(event)">-</button>
		</div>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		renderWizard:function(wizardsConfig,stepsData,stepIndex){//stepsData:步骤数据集，  stepIndex：步骤序号 从0开始
			var curRegion = this;
			
			if(stepsData==null){
				stepsData = [];
			}
			if(stepIndex==null)stepIndex=0;
			
			curRegion.wizardsConfig = wizardsConfig;//配置
			curRegion.stepsData = stepsData;//数据

			curRegion.gotoStep(stepIndex);
		},
		gotoStep:function(stepIndex){
			var curRegion = this;
			
			var stepAmount = curRegion.wizardsConfig.steps.length;
			if(stepIndex<0)stepIndex=0;
			if(curRegion.stepIndex==stepIndex)return;
			
			if(curRegion.curStepRegion!=null){//暂存step 数据
				var onSaveFunc = curRegion.curStepRegion.onSave;
				if(typeof(onSaveFunc)==="function"){
					var stepData = onSaveFunc.call(curRegion.curStepRegion); 
					curRegion.stepsData[curRegion.stepIndex] = stepData;
				}
			}
			
			if(stepIndex>(stepAmount-1)){
				if(typeof(curRegion.wizardsConfig.onSubmit)==="function"){
					curRegion.wizardsConfig.onSubmit(curRegion.stepsData,curRegion);
				}
				return;
			}
			
			curRegion.stepIndex = stepIndex;
			
			var curStepRegionPromise = RS.loadRegion(curRegion.find(".steps"),RS.appendParaMapToUrl(curRegion.wizardsConfig.steps[stepIndex],curRegion.paraMap),curRegion.regionId+stepIndex);
			curStepRegionPromise.done(function(stepRegion){
				var stepData = curRegion.stepsData[stepIndex];
				
				var onShowFunc = stepRegion.onShow;
				if(typeof(onShowFunc)==="function"){
					onShowFunc.call(stepRegion,stepData); 
				}
				
				curRegion.curStepRegion = stepRegion;//当前Step Region赋值
				curRegion.showOrHideBtn(stepIndex);
			})
		},
		showOrHideBtn:function(stepIndex){
			var curRegion = this;
			var stepAmount = curRegion.wizardsConfig.steps.length;
			
			var preLabel = curRegion.curStepRegion.preLabel;
			var nextLabel = curRegion.curStepRegion.nextLabel;
			
			curRegion.find(".next").text(nextLabel);
			if(stepIndex==0){
				curRegion.find(".pre").addClass("hidden");
			}
			else{
				curRegion.find(".pre").text(preLabel).removeClass("hidden");
			}
		},
		next:function(event){
			var curRegion = RS.getRegionByEvent(event);
			var funcResult = null;
			var func = curRegion.curStepRegion.next;
			if(typeof(func) === "function"){
				funcResult = func.call(curRegion.curStepRegion); 
			}
			else{
				console.warn("next方法不存在");
			}
			
			if(funcResult==null|| funcResult==true) {
				var stepIndex = curRegion.stepIndex+1;
				curRegion.gotoStep(stepIndex);
				return;
			}
			if(funcResult==false) return;
			
			if(typeof funcResult.done ==="function"){
				funcResult.done(function(){
					var stepIndex = curRegion.stepIndex+1;
					curRegion.gotoStep(stepIndex);
				})
			}
		},
		pre:function(event){
			var curRegion = RS.getRegionByEvent(event);
			var funcResult = null;
			
			var func = curRegion.curStepRegion.pre;
			if(typeof(func) === "function"){
				funcResult = func.call(curRegion.curStepRegion);
			}
			else{
				console.warn("pre方法不存在");
			}
			
			if(funcResult==null|| funcResult==true) {
				var stepIndex = curRegion.stepIndex-1;
				curRegion.gotoStep(stepIndex);
				return;
			}
			if(funcResult==false) return;
			
			if(typeof funcResult.done ==="function"){
				funcResult.done(function(){
					var stepIndex = curRegion.stepIndex+1;
					curRegion.gotoStep(stepIndex);
				})
			}
		},
		afterRenderData:function(){
			var curRegion = this;
			if(!curRegion.inited){
				curRegion.inited = true;
				//合并paraMap
				
				var paraMap = new HashMap();
				var tmpParaMap = RS.getOuterRegion(curRegion).paraMap;
				var tmpKeySet = tmpParaMap.keySet();
				for(var i = 0 ; i <tmpKeySet.length ;i++){
					paraMap.put(tmpKeySet[i],tmpParaMap.get(tmpKeySet[i]));
				}
				tmpParaMap = curRegion.paraMap;
				tmpKeySet = tmpParaMap.keySet();
				for(var i = 0 ; i <tmpKeySet.length ;i++){//优先使用本组件的paraKey
					paraMap.put(tmpKeySet[i],tmpParaMap.get(tmpKeySet[i]));
				}
				
				curRegion.paraMap = paraMap;
			}
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderWizard = REGION.renderWizard;
	staticRegion.gotoStep = REGION.gotoStep;
	staticRegion.showOrHideBtn = REGION.showOrHideBtn;
	staticRegion.renderRegion();
})
</script>