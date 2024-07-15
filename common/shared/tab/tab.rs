<style type="text/css">
#REGION.tab{
	height: 100% !important;
	position: relative;
	/*padding-top: 32px;*/
}

#REGION .tab-nav{	
	overflow: hidden;
	padding: 1px;
	position: absolute;
    top: 0;
}


#REGION .tab-regions{
	padding-top: 2.5rem;
	overflow:hidden;
	height: 100%;
}

#REGION .tab-nav-item{
	float: left;
	text-align: center;
/* 	line-height: 1.875rem; */
/* 	background: #f2f2f2; */
/* 	border: 1px solid; */
    margin: -1px;
    border-radius: 2px;
}

#REGION .tab-item{
	height: 100%;
    overflow: auto;
}
#REGION .tab-img{
	width: 1.25rem;
	padding-left: 0.3125rem;
}

</style>

<div id="REGION" class="hidden tab">
	<div class="tab-nav">
	
	</div>
	
	<div class="tab-regions">
	</div>

</div>	


<script type="text/javascript">
var REGION = {
		renderContent:function(){
			//var paraName = this.paraMap.get("paraName");
			var curRegion = this;
			if(!curRegion.inited){
				//禁止滚动条
				$(RegionUtil.getOuterRegion(curRegion).getRegionDivElem()).parent().parent().addClass("scrollbar-type1");

				curRegion.inited = true;
				curRegion.refreshRegion = REGION.refreshRegion;
				
				curRegion.gotoNextTab = function(){
					for(var i = 0 ; i <curRegion.tabsInfo.length ; i++){
						if(curRegion.currentTabName==curRegion.tabsInfo[i].name){
							if(i!=(curRegion.tabsInfo.length-1)){//没有到达最后一个tab
								REGION.switchTabByTabName(curRegion,curRegion.tabsInfo[i+1].name);
								break;
							}
						}
					}
				};
				curRegion.gotoPreTab = function(){
					for(var i = 0 ; i <curRegion.tabsInfo.length ; i++){
						if(curRegion.currentTabName==curRegion.tabsInfo[i].name){
							if(i!=0){//没有到达第一个tab
								REGION.switchTabByTabName(curRegion,curRegion.tabsInfo[i-1].name);
								break;
							}
						}
					}
				};
			}
		},
		getTabRegionByName:function(tabName){
			return this.tabRegions[tabName];
		},
		refreshRegion:function(){
			var curRegion = this;
			setTimeout(function(){
				curRegion.inited = true;
				curRegion.loadedTabs = {};
				curRegion.tabRegions ={};
				curRegion.getTabRegionByName = REGION.getTabRegionByName.bind(curRegion)
				
				var appendHtml="";
				var appendHtml2="";
				var tmpInfo = null;
				curRegion.tabMap = new HashMap();//tab名-序号
				for(var i = 0 ; i <curRegion.tabsInfo.length ; i++){
					tmpInfo = curRegion.tabsInfo[i];
					appendHtml+='<div class="'+tmpInfo.name+' tab-item scrollbar-type1 hidden"></div>';
					
					appendHtml2+='<a class="tab-nav-item " draggable="false" tab-name="'+tmpInfo.name+'" href="javascript:void(0)" onclick="REGION.switchTab()">';
					
					if(tmpInfo.label.indexOf('?')!=-1){//特殊处理
						appendHtml2+=tmpInfo.label.split('?')[0];
						let layerName=tmpInfo.label.split('?')[1];
						//图标
						// appendHtml2+='<i class="fa fa-map tab-img" title="'+layerName+'" onclick="REGION.selectEye(event,\''+layerName+'\')"></i>';
						appendHtml2+='<img src="/images/mapclick.png" class="tab-img show-time" onclick="REGION.selectEye(event,\''+layerName+'\')">';
					}else{
						appendHtml2+=tmpInfo.label;
					}
					appendHtml2+='</a>';
					
					curRegion.tabMap.put(tmpInfo.name,i);
				}
				
				RegionUtil.setInnerHtml(curRegion,curRegion.find(".tab-nav")[0],appendHtml2);//导航区域
				RegionUtil.setInnerHtml(curRegion,curRegion.find(".tab-regions")[0],appendHtml);//内容区域
				
				var regionWidth = $(curRegion.getRegionDivElem()).width();//region 宽度  width();   innerWidth();  outerWidth(); outerWidth(true);
				var regionHeight= $(window).height();
				var tabsObjArray = curRegion.find(".tab-nav-item");
				for(var i = 0 ; i <tabsObjArray.length ; i++){
					var tmpObj = $(tabsObjArray[i]);
					tmpObj.css("width",((regionWidth-3)/tabsObjArray.length)+"px");
					//tmpObj.css("height",(regionHeight-50)+"px");
				}
				//-------------------------------------------------------
				REGION.switchTabForTarget(curRegion.find(".tab-nav-item")[0]);//加载第一个tab
			})
		},
		unloadTab:function(curRegion,tabName){
			var tabRegion = curRegion.tabRegions[tabName];
			if(tabRegion==null)return;
			
			tabRegion.release();
			
			//$(tabRegion.getRegionDivElem()).parent().remove();
			curRegion.tabRegions[tabName] = null;
			curRegion.loadedTabs[tabName]=null;
		},
		getTabData:function(curRegion,tabName){
			var targetTabIndex = curRegion.tabMap.get(tabName);
			return curRegion.tabsInfo[targetTabIndex];
		},
		switchTabByTabName:function(curRegion,tabName,paraMap){//paraMap switch参数
			//var regionTabsDiv = curRegion.find(".REGION_TABS_NAV");//navigation
			if (curRegion.onClick!=null){
				curRegion.onClick.bind(curRegion)(tabName);
			}
			
			if(paraMap==null){
				paraMap = new HashMap();

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
			}
			
			var tabRegionsDiv = curRegion.find(".tab-regions");//tabs
			if(curRegion.currentTabName!=tabName){
				if(curRegion.currentTabName!=null){
					var tabData = REGION.getTabData(curRegion,curRegion.currentTabName);
					var cacheAble = tabData.cacheAble;
					if(cacheAble==null)cacheAble=true;
					if(tabData.cacheAble==false){
						REGION.unloadTab(curRegion,curRegion.currentTabName);
					}
				}
				

				var targetTabIndex = curRegion.tabMap.get(tabName);
				var targetTabHolder = tabRegionsDiv.find("."+tabName);
				
				var paraMapChanged = false;
				var newSwitchParaStr = RegionUtil.appendParaMapToUrl("",paraMap);
				if(newSwitchParaStr!=curRegion.newSwitchParaStr)
					paraMapChanged = true;
				curRegion.newSwitchParaStr = newSwitchParaStr;
				
				
				if(paraMapChanged||curRegion.loadedTabs[tabName]==null){//如果参数改变或者未加载 则重新加载
					// RegionUtil.loadingStart();
					var regionUrl = curRegion.tabsInfo[targetTabIndex].tabRegion;
					if(paraMap!=null){
						regionUrl = RegionUtil.appendParaMapToUrl(regionUrl,paraMap);
					}
					var def = RegionUtil.loadRegion(targetTabHolder,regionUrl,curRegion.regionId+"_"+tabName);
					def.done(function(loadedRegion){
						curRegion.tabRegions[tabName] = loadedRegion;//存入tab的region引用
						curRegion.loadedTabs[tabName] = tabName;
						curRegion.currentTabName = tabName;
						if (curRegion.onSwitch!=null){
							curRegion.onSwitch.bind(curRegion)(loadedRegion);
						}
						// RegionUtil.loadingComplete();
					}) 
				}
				else{
					curRegion.currentTabName = tabName;
				}
				
				//animations
				var tabsObjArray = curRegion.find(".tab-item");
				for(var i = 0 ; i <tabsObjArray.length ; i++){
					var tmpObj = $(tabsObjArray[i]);
					if(tmpObj.hasClass(tabName))tmpObj.removeClass("hidden");
					else tmpObj.addClass("hidden");
				}
			} 
		},
		switchTabForTarget:function(tabNavItem){
			$(tabNavItem).addClass("tab-nav-active")
			$(tabNavItem).siblings().removeClass("tab-nav-active")
			var curRegion = RegionUtil.getRegionByElem(tabNavItem);
			var tabName = tabNavItem.getAttribute("tab-name");
			REGION.switchTabByTabName(curRegion,tabName);
			
			
		},
		switchTab:function(event){
			var targetElem = RegionUtil.getEventTarget(event);
			var tabItem = $(targetElem).closest(".tab-nav-item")[0];
			REGION.switchTabForTarget(tabItem);
		},
		selectEye:function(event,layerName){
			// debugger
			event.stopPropagation();
			let curRegion=RegionUtil.getRegionByEvent(event);
			curRegion.publishEvent("mapListener",{layerName:[layerName]});
		},
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.renderContent;
	staticRegion.renderRegion();
})
</script>

