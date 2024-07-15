<!-- 通用screen 屏组件 -->
<style type="text/css">
#REGION{
	padding-left:19rem;
	position: relative;
	transition: padding-left .3s;
}

#REGION>.menuHolder-box{
	position: absolute;
	width:19rem;
 	top:0;
	left: 0;
	bottom: 0;
}
#REGION .menuHolder{
	height:100%;
	padding-bottom: 3rem;
}
#REGION>.contentHolder{
	position: relative;
	width: 100%;
	height: 100%;
	/*overflow: auto;*/
}
#REGION .res-arrow{
	position: absolute;
	bottom: 0;
	height:3rem;
	left: 0;
	right: 0;
	display: flex;
	align-items: center;
	justify-content: center;
	font-size:1.25rem;
	z-index: 0;
	cursor: pointer;

}
#REGION .res-arrow-box{
	width: 100%;
	position: relative;
	height: 3rem;
	display: flex;
	justify-content: center;
	align-items: center;
}
#REGION .res-arrow-box > i{
	position: absolute;
	width: 2.5rem;
	height: 2rem;
	display: flex;
	align-items: center;
	justify-content: center;
	left: 27%;
	font-size: 1rem;
	top: 0.6rem;
}
#REGION .res-arrow > i:hover{
	transform:initial;
}
#REGION .res-arrow-box >  span{
	font-size: 0.875rem;
}

/*#REGION .rote-class-right{*/
/*	padding-right: 1.5rem;*/
/*}*/

</style>

<div id="REGION" class="hidden menu-toggle">
	<div class="menuHolder-box">
		<div class="menuHolder"></div>
		<div>
			<div class="res-arrow">
				<div class="res-arrow-box">
					<i class="fa-regular fa-chevrons-left">
					</i>
					<span class="collapse-title">折叠菜单栏</span>
				</div>


			</div>
		</div>
	</div>
	<div class="contentHolder"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			var rsCloudMainRegion = RS.getOuterRegion(RS.getOuterRegion(RS.getOuterRegion(curRegion)));
			var childNodeData = null;
			var childs = rsCloudMainRegion.treeRootNodeData.childs;
			
			var curScreenName = curRegion.paraMap.get("screenName");
			for(var i = 0 ; i<childs.length ;i++){
				childNodeData = childs[i];
				if(childNodeData.value==curScreenName){
					break;
				}
			}	
			REGION.nodeReg = {};
			var listData = REGION.convertTreeToList(childNodeData);
			var selectedMenuNodeId = curRegion.paraMap.get("menuId");//url传递进来的mennuId
			var loadContentRegion = RegionUtil.loadRegion(curRegion.find(".contentHolder"),"common/shared/index/screenStyle/content.rs");
			loadContentRegion.done(function(contentRegion){
				curRegion.contentRegion = contentRegion;
				
				var promise = RegionUtil.loadRegion(curRegion.find(".menuHolder"),"common/shared/index/screenStyle/menu.rs");
				promise.done(function(loadedRegion){
	            	for(var i = 0 ; i <listData.length ;i++){
	            		listData[i].expand = true;//默认全部展开
	            	}
					loadedRegion.renderMenu(listData,selectedMenuNodeId);
				})
			})
			var navIcon = curRegion.find(".res-arrow-box");
			navIcon.click(function(){
				if(navIcon.hasClass("rote-class-right")){
					curRegion.restore();
					navIcon.find(".collapse-title").show()
					navIcon.find(".fa-chevrons-left").css({'left':'27%'})
				}
				else{
					curRegion.minimize();
					navIcon.find(".collapse-title").hide()
					navIcon.find(".fa-chevrons-left").css({'left':'initial'})
				}
			})


		},
	minimize:function(){//页面框架缩小
		let curRegion = this;
		var headerRegion =RS.getOuterRegion(RS.getOuterRegion(RS.getOuterRegion(curRegion)));
		var logoRegion =headerRegion.headerRegion.logoRegion
		headerRegion.minimize();
		 logoRegion.minimize();
		curRegion.find(".res-arrow-box").removeClass("rote-class-left").addClass("rote-class-right");
		window.minimizedAdmin = true;
	},
	restore:function(){//页面框架还原
		let curRegion = this;
		var headerRegion =RS.getOuterRegion(RS.getOuterRegion(RS.getOuterRegion(curRegion)));
		var logoRegion =headerRegion.headerRegion.logoRegion

		headerRegion.restore();
		 logoRegion.restore();
		curRegion.find(".res-arrow-box").removeClass("rote-class-right").addClass("rote-class-left");
		window.minimizedAdmin = false;
	},
		convertTreeToList:function(treeNodeData){
			var nodeList = [];
			if(treeNodeData.childs!=null){
				for(var i = 0 ; i <treeNodeData.childs.length ;i++){
					//treeNodeData.childs[i].depth--;
					if(treeNodeData.childs[i].depth==2)treeNodeData.childs[i].parentNodeId="0";//重置父节点
					if(REGION.nodeReg[treeNodeData.childs[i].id]==null){
						nodeList.push(treeNodeData.childs[i]);
						REGION.nodeReg[treeNodeData.childs[i].id]=true;
					}
					nodeList = nodeList.concat(REGION.convertTreeToList(treeNodeData.childs[i]));
				}
			}
			return nodeList;
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.minimize=REGION.minimize;
	staticRegion.restore=REGION.restore;
	staticRegion.renderRegion();
})
</script>