<div id="REGION" class="hidden">
	<div class="wizard" style="	height: 100%;"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			var clusterId = curRegion.paraMap.get("clusterId");
			
			var wizardRegionPromise = RS.loadRegion(curRegion.find(".wizard"),"common/shared/wizard/wizard.rs");
			wizardRegionPromise.done(function(loadedRegion){
				var wizardsConfig = {
						steps: [
								'common/shared/wizard/sample/step1.rs',
								'common/shared/wizard/sample/step2.rs',
								'common/shared/wizard/sample/step3.rs',
							 ],
						 onSubmit:function(wizardData,wizardRegion){//提交步骤整体数据
								console.log("onSubmit")
								console.log(wizardData)
								console.log(wizardRegion)
						 }
				}
				loadedRegion.renderWizard(wizardsConfig);
			})
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>