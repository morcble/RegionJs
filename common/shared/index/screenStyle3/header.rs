<style type="text/css">
#REGION>.module-section{
	position: relative;
	width: 100%;
	height: 100%;
}

</style>

<div id="REGION" class="hidden header-toggle">
	<div class="logo-section"></div>
	<div class="module-section"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			var systemCode = SaasUtil.getSystemCode();
			var loadAdminLogPromise = RS.loadRegion(curRegion.find(".logo-section"),"sys/"+systemCode+"/index/adminlogo.rs");
			loadAdminLogPromise.done(function(loadedRegion){
				if(loadedRegion==null){
					loadAdminLogPromise = RS.loadRegion(curRegion.find(".logo-section"),"common/shared/index/common/adminlogo.rs");
					loadAdminLogPromise.done(function(loadedRegion){
						curRegion.logoRegion = loadedRegion;
					})
				}
				else{
					curRegion.logoRegion = loadedRegion;
				}
			})
			
			var loadNavPromise = RegionUtil.loadRegion(curRegion.find(".module-section"),"common/shared/index/screenStyle3/module.rs");
			loadNavPromise.done(function(loadedRegion){
				curRegion.moduleRegion = loadedRegion;
			})
			return loadNavPromise;
		},
		setModuleItems:function(moduleArray){
			var curRegion = this;
			curRegion.moduleRegion.setModuleItems(moduleArray);
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.setModuleItems = REGION.setModuleItems;
	staticRegion.renderRegion();
})
</script>