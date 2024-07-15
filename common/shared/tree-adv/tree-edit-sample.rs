<style type="text/css">
#REGION>.content{
	padding: 6px;
	height: 100%;
}

#REGION>.tree-holder{
	padding: 6px;
	height: 100%;

}
</style>

<div id="REGION" class="hidden match-parent">
	<div class="col-xs-2 tree-holder"></div>
	<div class="col-xs-10 content"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			
			if(!curRegion.inited)curRegion.inited = true;else return;
			
			RegionUtil.loadingStart();
			//树的租户ID
			curRegion.treeSpId = null;//如果为null则取登录用户的spId
			//树的模块
			curRegion.treeModule = "module";
			//添加节点的弹窗组件
			curRegion.addNodeRegion = "common/shared/tree-adv/sample/addNodeRegion.rs";
			//编辑节点的弹窗组件
			curRegion.editNodeRegion = "common/shared/tree-adv/sample/editNodeRegion.rs";
			
			//树标题
			curRegion.treeTitle = "产品分类";
			//初始化右键菜单
			curRegion.contextMenu = [{
				"label" : "新增一级分类",
				"onclick" : "REGION.contextMenuClick"
			}];
			
			//READ-ONLY BEGIN
			//编辑节点的弹窗组件
			curRegion.treeRes = Config.backendPath+"/RCS_Basic/treenode/list-without-pagination";
			curRegion.addLevelOne = REGION.addLevelOne.bind(curRegion);
					
			var paraMap = new HashMap();
			var promise1 = RegionUtil.loadRegion(curRegion.find(".tree-holder"),RegionUtil.appendParaMapToUrl("common/shared/tree-adv/tree-edit/tree-holder.rs",paraMap),curRegion.regionId+"_treeholder");
			promise1.done(function (treeHolderRegion) {
				curRegion.treeHolderRegion=treeHolderRegion;
				var treeRegionId = treeHolderRegion.treeRegionId;
				var treeRegion = RegionUtil.getRegionById(treeRegionId);
				treeRegion.nodeClick = REGION.nodeClick;//绑定节点点击事件
				treeRegion.contextMenu = curRegion.contextMenu;
				treeRegion.renderMenu(treeRegion.treeData);
			});
			
			$.when(promise1).done(function(){
				RegionUtil.loadingComplete();
			}) 
			//READ-ONLY END
		},
		addLevelOne:function(treeHolderRegion){//添加第一级
			var curRegion = RegionUtil.getOuterRegion(treeHolderRegion);
			
			var title = '新增一级分类';
			var paraMap = new HashMap();
			paraMap.put("spId",curRegion.treeSpId);
			paraMap.put("module",curRegion.treeModule);
			paraMap.put("pid","0");
			paraMap.put("depth",0);
			paraMap.put("treeRegionId",treeHolderRegion.treeRegionId);
			RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl(curRegion.addNodeRegion,paraMap),title,2,null,null,600,550);
		},
		contextMenuClick: function(event,index,label){//背景右键
			console.log(index)
			console.log(label)
			var treeRegion = RegionUtil.getRegionByEvent(event);
			treeRegion.hideMenu();
			var treeHolderRegion = RegionUtil.getOuterRegion(treeRegion);
			REGION.addLevelOne(treeHolderRegion);
		},
		nodeMenuClick:function(event,index,label){//右键菜单
			console.log(index)
			console.log(label)
			var treeRegion = RegionUtil.getRegionByEvent(event);
			treeRegion.hideMenu();
			var treeHolderRegion = RegionUtil.getOuterRegion(treeRegion);
			console.log(action);
			//treeRegion.removeNode(treeRegion.selectedNodeData.id);
			//treeRegion.refreshNode(selectedNodeData);
		},
		nodeClick:function(nodeData){//节点点击
			var treeRegion = this;
	    	treeRegion.nodeMenu = [//修改鼠标右击菜单
				{
					"label" : "新增三级分类",
					"onclick" : "REGION.nodeMenuClick"
				},
				{
					"label" : "编辑",
					"onclick" : "REGION.nodeMenuClick"
				},
				{
					"label" : "删除",
					"onclick" : "REGION.nodeMenuClick"
				}
			];

	    	var treeRegionId = treeRegion.regionId;
	    }
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>