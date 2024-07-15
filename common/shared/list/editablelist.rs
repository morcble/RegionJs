<!-- 可编辑表格包装组件 -->
<style type="text/css">


</style>

<div id="REGION" class="hidden">
		<div class="list-holder">
			
		</div>
		
				<div class="form-body">
					<div class="col-xs-12 col-md-12">
						<button class="btn"  data-type="primary" normal onclick="REGION.addRow(event)"><message key="global_msg.add"></message></button>
						
					</div>
				</div>
</div>



<script type="text/javascript">
Array.prototype.remove = function(dx) {
　　if(isNaN(dx) || dx > this.length){
　　　　return false;
　　}
 
　　for(var i = 0, n = 0; i < this.length; i++) {
　　　　if(this[i] != this[dx]) {
　　　　　　this[n++] = this[i];
　　　　}
　　}
　　this.length -= 1;
};


var REGION = {
		addRow:function(event){
			var editablelistRegion = RegionUtil.getRegionByEvent(event);
			
			var rowId = ""+RegionUtil.getUUID();
			var rowRegionId = "REGION"+RegionUtil.getUUID();
			var curIndex = "0";
			
			var rowRegionHolderHtml = '<div class="list-item list-item'+rowRegionId+'" row-id="'+rowId+'" row-region-id="'+rowRegionId+'" row-index="'+curIndex+'"></div>';
			
			var listHolder = editablelistRegion.find(".list-holder");
			RegionUtil.appendHtml(editablelistRegion,listHolder[0],rowRegionHolderHtml);
			
			var paraMap = new HashMap();
			paraMap.put("rowDataId",rowId);
			var promise = RegionUtil.loadRegion(editablelistRegion.find(".list-item"+rowRegionId),RegionUtil.appendParaMapToUrl(editablelistRegion.rowRegion,paraMap),rowRegionId);
		},
		afterRenderData:function(){
			var editablelistRegion = this;
			
			if(!editablelistRegion.inited){
				editablelistRegion.inited = true;
				editablelistRegion.rowCount = 0;
				
				editablelistRegion.renderListData = REGION.renderListData.bind(editablelistRegion);
				editablelistRegion.getListData = REGION.getListData.bind(editablelistRegion);
				
				var rowRegion = editablelistRegion.paraMap.get("rowRegion");
				if(rowRegion==null||rowRegion.trim()==""){
					RegionUtil.alert("rowRegion is undefined");
					return;
				}
				
				editablelistRegion.rowRegion = rowRegion;//赋值１
				//init with datas
				var listData = editablelistRegion.paraMap.get("listData");
				if(listData!=null&&listData.trim()!=""){
					editablelistRegion.renderListData(JSON.parse(listData));
				}
			}
		},
		getListData:function(){
			var editablelistRegion = this;
			var listHolder = editablelistRegion.find(".list-holder");
			var listItems = listHolder.find(".list-item");
			var result = [];
			for(var i = 0 ; i<listItems.length;i++){
				var rowRegionId = listItems[i].getAttribute("row-region-id");
				var rowRegion = RegionUtil.getRegionById(rowRegionId);
				result.push(rowRegion.getFormData());
			}
			return result;
		},
		renderListData:function(listData){//刷新行
			var editablelistRegion = this;
			
			var listHolder = editablelistRegion.find(".list-holder");
			var listItems = listHolder.find(".list-item");
			var listDataRegister = {};
			var index = 0;
			for(var i = 0;i<listData.length;i++){
				if(listData[i].id==null)listData[i].id = ""+RegionUtil.getUUID();
				listData[i].index = index;
				index++;
				listDataRegister[""+listData[i].id] = listData[i];
			}
		
			for(var i = 0 ; i<listItems.length ;i++){
				var listItemObj = $(listItems[i]);
				var rowId = listItemObj.attr("row-id");
				if(listDataRegister[""+rowId]==null){//已经被删除
					listItemObj.addClass("to-removed");//标记为待删除
				}
				else{
					var rowRegionId = listItemObj.attr("row-region-id");
					var rowRegion = RegionUtil.getRegionById(rowRegionId);
					rowRegion.setFormData(listDataRegister[""+rowId]);//刷新
				}
				delete listDataRegister[""+rowId];//从注册表删除
			}
			
			listHolder.children(".to-removed").remove();
			
			//显示新增的行
			var newRowDatas = [];//保存新增加的列
			for(x in listDataRegister){
				newRowDatas.push(listDataRegister[x]);
			}
			newRowDatas.sort(function(a,b){return a.index-b.index});
			
			for(var k = 0 ; k <newRowDatas.length ;k++){
				var listItems = listHolder.find(".list-item");
				var rowData = newRowDatas[k];
				var curIndex = rowData.index;
				var rowId = rowData.id;
				if(rowId==null)RegionUtil.alert("rowId is null")
				var rowRegionId = "REGION"+RegionUtil.getUUID();
				
				var rowRegionHolderHtml = '<div class="list-item list-item'+rowRegionId+'" row-id="'+rowId+'" row-region-id="'+rowRegionId+'" row-index="'+curIndex+'"></div>';
				
				if(listItems.length==0){
					RegionUtil.appendHtml(editablelistRegion,listHolder[0],rowRegionHolderHtml);
				}
				else if(listItems.length==1){
					var rowIndex = listItems.attr("row-index");
					if(rowIndex<curIndex){
						RegionUtil.appendHtml(editablelistRegion,listHolder[0],rowRegionHolderHtml);
					}
					else{
						RegionUtil.insertBefore(editablelistRegion,editablelistRegion.find(".list-item"+rowIndex)[0],rowRegionHolderHtml);
					}
				}
				else{
					if(curIndex<listItems[0].getAttribute("row-index")){
						RegionUtil.insertBefore(editablelistRegion,listItems[0],rowRegionHolderHtml);
					}
					else if(curIndex>listItems[listItems.length-1].getAttribute("row-index")){
						RegionUtil.insertAfter(editablelistRegion,listItems[listItems.length-1],rowRegionHolderHtml);
					}
					else{
						for(var i = 0 ; i<listItems.length ;i++){
							if(i==listItems.length-1)break;
							
							var startIndex = listItems[i].getAttribute("row-index");
							var endIndex = listItems[i+1].getAttribute("row-index");
							if(startIndex<curIndex && curIndex<endIndex){
								RegionUtil.insertBefore(editablelistRegion,listItems[i+1],rowRegionHolderHtml);
								break;
							}
						}
					}
				}
				
				var paraMap = new HashMap();
				paraMap.put("rowDataId",rowData.id);
				var promise = RegionUtil.loadRegion(editablelistRegion.find(".list-item"+rowRegionId),RegionUtil.appendParaMapToUrl(editablelistRegion.rowRegion,paraMap),rowRegionId);
				promise.done(function(rowRegion){
					var rowDataId = rowRegion.paraMap.get("rowDataId");
					for(var l = 0 ; l <newRowDatas.length ;l++){
						if(rowDataId==newRowDatas[l].id){
							rowRegion.setFormData(newRowDatas[l]);
							break;
						}
					}
					
				})
			}
			
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>