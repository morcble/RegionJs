<style>
#REGION .fa{
	min-width: 1.2rem;
	text-align: center;
}
#REGION .stackmenuHolder{
	height: 100%;
	width: 100%;
}
</style>
<div id="REGION" class="hidden match-parent stakmenu-bg">
	<div class="stackmenuHolder"></div>
</div>


<script type="text/javascript">
var REGION = {
		lastClickTime:-1,
		renderMenu:function(menuData,selectedMenuNodeId){
			var curRegion = this;
			
			curRegion.minimize=function(){
				curRegion.stackmenuRegion.minimize();
			}
			
			curRegion.restore=function(){
				curRegion.stackmenuRegion.restore();
			}
			
			var menuUrl = "common/shared/menu/stackmenu.rs";
			if(selectedMenuNodeId!=null){
				menuUrl = "common/shared/menu/stackmenu.rs?selectedNodeId="+selectedMenuNodeId;
			}
			
			 var def = RegionUtil.loadRegion(curRegion.find(".stackmenuHolder"),menuUrl,curRegion.regionId+"_stackmenu");
             def.done(function(stackmenuRegion){
            	stackmenuRegion.nodeClick = REGION.nodeClick;
            	stackmenuRegion.renderMenu(menuData);
            	curRegion.stackmenuRegion=stackmenuRegion;
            });
             
			$(this.getRegionDivElem()).parent().parent().addClass("stackmenu-cell").addClass("no-scrollbar");
		},
		nodeClick:function(nodeData){
			var curRegion = RS.getOuterRegion(this);
			var screenRegion = RS.getOuterRegion(curRegion); //eg.  screenRegion.rs
			var contentRegion = screenRegion.contentRegion;
			var now = new Date().getTime();	
			if(REGION.lastClickTime!=-1&&((REGION.lastClickTime+200)>now))return;

			setTimeout(function(){
				//修改state
				var tmpOrigin = location.origin;
				var tmPathname = location.pathname;
				var tmpHref = location.href;
				var tmpSearch = location.search;
				var tmpHash = location.hash;
				
				var urlHashMap = RS.getUrlParaMap();
				var urlMenuId = urlHashMap.get("menuId");
				if(urlMenuId==null){
					//添加menuId
					urlHashMap.put("menuId",nodeData.id);
					var newSearch = RS.appendParaMapToUrl(location.search,urlHashMap);
					var newUrl = location.protocol + "//" + location.host+ location.pathname + newSearch + location.hash;
					var screenUUID = "global_"+RegionUtil.UUID();
					history.replaceState(screenUUID, "", newUrl);
				}
				else{
					if(urlMenuId!=nodeData.id){
						//tmpHash = tmpHash.replaceAll(/menuId=[0-9]+/,"");
						//var newUrl = tmpOrigin+tmPathname+ tmpSearch.replaceAll(/menuId=[0-9]+/,"menuId="+nodeData.id + tmpHash);
						
						var newUrl = location.href.replaceAll(/menuId=[0-9]+/,"menuId="+nodeData.id);
						
						var screenUUID = "global_"+RegionUtil.UUID();
						history.replaceState(screenUUID, "", newUrl);
					}
				}
				
				var paraMap = RS.getUrlParaMap();
				var toOpenRegion = null;
				
				var targetUrl = nodeData.value;
				if(targetUrl==null){
					return;
				}
				else if(targetUrl.startWith("http")){
					window.open(targetUrl,"_blank");
					return;
				}
				else if(targetUrl.endWith(".rs")||targetUrl.indexOf(".rs?")!=-1){
					//donothing
				}
				else{
					return;
				}
				
				if(targetUrl!=null)
					 toOpenRegion = RegionUtil.appendParaMapToUrl(targetUrl,paraMap);
		
				if(targetUrl!=null&&targetUrl.trim()!=""){
					var targetRegion = contentRegion;
					if(targetRegion==null){
						setTimeout(function(){
							REGION.nodeClick(nodeData);
						});
					}
					else {
						var tmpLabel = nodeData.label;
						if(nodeData.depth!=null&&nodeData.depth>1&&nodeData.parent!=null){
							tmpLabel = nodeData.parent.label+"一"+tmpLabel;
						}
						
						targetRegion.loadTab(nodeData.id,toOpenRegion,tmpLabel,false);
					}
				}
					
				REGION.lastClickTime = now;
			})
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.renderMenu = REGION.renderMenu;
	staticRegion.renderRegion();
})
</script>

