<style type="text/css">
	#REGION{
		padding: 0 0.75rem;
	}
#REGION .common-tree{
 	height:calc(100% - 4rem);
    overflow: auto;
	width: 100%;
	float: left;
}
#REGION .form-search{
	/*padding: 0.125rem;*/
}
#REGION .form-info{
	display: flex;
	justify-content: space-between;
	align-items: center;
}
</style>

<div id="REGION" class="hidden">
			<div class="form-info">
				<div class="title form-text"></div>
					<i class="fa-solid fa-plus grid-plus" onclick="REGION.addLevelOne(event);" title="add"></i>
			</div>
			<div class="form-search">
				<input type="text" class="form-control" placeholder="输入关键字搜索" oninput="REGION.filterTree(event)">
			</div>

			<div class="common-tree"></div>
		</div>


<script type="text/javascript">
var REGION = {
		filterTree:function(event){
			var targetElem = RegionUtil.getEventTarget(event);
			var curRegion = RegionUtil.getRegionByElem(targetElem);

			var treeRegion = RegionUtil.getRegionById(curRegion.treeRegionId);
			treeRegion.filterNode($(targetElem).val());
		},
		afterRenderData:function(){
			var curRegion = this;
			var outerRegion = RegionUtil.getOuterRegion(curRegion);
			
			if(!curRegion.inited)curRegion.inited = true;else return;

			curRegion.find(".title").text(outerRegion.treeTitle);

			var treeRegionId = curRegion.regionId+"_tree";
			curRegion.treeRegionId = treeRegionId;
			
			var keys = curRegion.paraMap.keySet();
			
			
			var reqObj = new Object();
			for(var i = 0 ; i<keys.length ;i++){
				reqObj[keys[i]] = curRegion.paraMap.get(keys[i]);
			}
			
			let selectedNodeId = curRegion.paraMap.get("selectedNodeId");
			/* 
			
			reqObj.spId = outerRegion.treeSpId;			
			reqObj.systemId = outerRegion.treeSystemId;
			reqObj.module = outerRegion.treeModule; */
			
			var regionLoadedDefer = new jQuery.Deferred();
			RegionUtil.call(outerRegion.treeRes,"POST",reqObj,function(serverResponse,dataPara){
				if(serverResponse.success){
					var listData = serverResponse.data;
					if(listData instanceof Array){
						//此处兼容旧的后端API
					}
					else{
						if(listData.list!=null)listData=listData.list;
					}
					
					for(var i = 0 ;i<listData.length;i++){
						listData[i].pid = listData[i].pid;
						listData[i].label = listData[i].label;
					}
					
					let treeUrl = "common/shared/tree/simple-tree-new.rs?showDots=false";
					if(selectedNodeId!=null){
						treeUrl +="&selectedNodeId="+selectedNodeId;
					}
					var def = RegionUtil.loadRegion(curRegion.find(".common-tree"),treeUrl,treeRegionId);
		            def.done(function(treeRegion){
		                treeRegion.treeData = listData;
		                regionLoadedDefer.resolve();
		            });
				}
			})
			return regionLoadedDefer.promise();
		},
		addLevelOne:function(event){
			var curRegion = RegionUtil.getRegionByEvent(event);
			var parentRegion = RegionUtil.getOuterRegion(curRegion);
			parentRegion.addLevelOne(curRegion);
		}
};

RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION");
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>