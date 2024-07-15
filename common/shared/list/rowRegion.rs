<style type="text/css">


</style>

<div id="REGION" class="hidden">
	<div class="form">
		<input region-attr="name" type="text" class="form-control region-editable" maxlength="30">
		<select region-attr="status" class="form-control region-editable" region-ds="EnableDisableInt">
		            <option value="" msgKey="global_msg.please_select"></option>
		</select>
		<button class="btn"  data-type="primary" normal onclick="REGION.deleteRow(event)">删除</button>
		<i class="fa  fa-chevron-up" onclick="REGION.moveUp(event)"></i>
		<i class="fa  fa-chevron-down" onclick="REGION.moveDown(event)"></i>
	</div>
	
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			var rowRegion = this;
			rowRegion.getFormData = REGION.getFormData.bind(rowRegion);
			rowRegion.setFormData = REGION.setFormData.bind(rowRegion);
			rowRegion.moveUp = REGION.moveUp.bind(rowRegion);
			rowRegion.moveDown = REGION.moveDown.bind(rowRegion);
		},
		moveUp:function(event){
			var rowRegion = RegionUtil.getRegionByEvent(event);
			var editablelistRegion = RegionUtil.getOuterRegion(rowRegion);
			
			var listItemObj = editablelistRegion.find(".list-holder").children(".list-item"+rowRegion.regionId);
			var prevItem = listItemObj.prev();
			if(prevItem.length==1){
				listItemObj.insertBefore(prevItem);
			}
		},
		moveDown:function(event){
			var rowRegion = RegionUtil.getRegionByEvent(event);
			
			var rowRegion = RegionUtil.getRegionByEvent(event);
			var editablelistRegion = RegionUtil.getOuterRegion(rowRegion);
			
			var listItemObj = editablelistRegion.find(".list-holder").children(".list-item"+rowRegion.regionId);
			var nextItem = listItemObj.next();
			if(nextItem.length==1){
				listItemObj.insertAfter(nextItem);
			}
		},
		deleteRow:function(event){
			var rowRegion = RegionUtil.getRegionByEvent(event);
			var editablelistRegion = RegionUtil.getOuterRegion(rowRegion);
			var listItemObj = editablelistRegion.find(".list-holder").children(".list-item"+rowRegion.regionId);
			listItemObj.remove();
		},
		getFormData:function(){
			return this.packFormData();
		},
		setFormData:function(rowData){
			this.renderRegionWithData(rowData);
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newFormRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>