<style type="text/css">

</style>

<div id="REGION" class="hidden">
	<div class="screenHolder" style="height:100%"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			var screenNodes = JSON.parse(curRegion.paraMap.get("screenNodes"));
			
		 	var def = RegionUtil.loadRegion(curRegion.find(".screenHolder"),RegionUtil.appendParaMapToUrl("common/shared/screen/screen-new.rs",this.paraMap),"mainscreen");
	 		def.done(function(loadedRegion){
	 			var screensInfo = [];
	 			for(var i = 0 ; i <screenNodes.length ;i++){
	 				var screenName = screenNodes[i].screenName;
	 				var url = screenNodes[i].url;
	 				if(screenName==null){
	 					alert("ScreenName is empty,Please make sure system menu is configured properly");
	 					console.error(screenNodes[i]);
	 					continue;
	 				}
	 				
	 				if(screenNodes[i].type=="rs"){
	 					screensInfo.push({'name':screenName,'screenRegion':url,'useCache':true });
	 				}
	 				else if(screenNodes[i].type=="plain"){
	 					screensInfo.push({'name':screenName,'screenRegion':'common/shared/index/screenStyle3/screenCommon.rs','useCache':true });
	 				}
	 			}
	 			
// 	 			var screensInfo = [
// 	 				{'name':'workstation','screenRegion':'common/shared/index/screenStyle/screenCommon.rs','useCache':true },
// 	 				{'name':'projectmanage','screenRegion':'common/shared/index/screenStyle/screenCommon.rs','useCache':true },
// 	 				{'name':'maintain','screenRegion':'common/shared/index/screenStyle/screenCommon.rs','useCache':true },
// 	 				{'name':'storage','screenRegion':'common/shared/index/screenStyle/screenCommon.rs','useCache':true },
// 	 				{'name':'fastentry','screenRegion':'common/shared/index/screenStyle/screenCommon.rs','useCache':true },
// 	 				{'name':'platformmanage','screenRegion':'common/shared/index/screenStyle/screenCommon.rs','useCache':true }
// 	 			];
	 			loadedRegion.onSwitch = function(screenName,screenRegion){//屏幕切换之后需要更新菜单
	 				console.log(screenName)
	 				//console.log(screenRegion.rcsFile)
	 				var adminMainRegion = RS.getOuterRegion(curRegion);
	 				adminMainRegion.headerRegion.moduleRegion.find(".nav_"+screenName).addClass("nav-active").siblings().removeClass("nav-active")
	 			}
	 			var ignoreParasForScreen = ["menuId"];
	 			loadedRegion.render(screensInfo,false,ignoreParasForScreen);
	 		});
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>