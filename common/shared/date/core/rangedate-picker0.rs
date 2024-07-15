<style type="text/css">
#REGION>div{
	display: inline-block;
	margin: 0px 2px;
	vertical-align: text-top;
}
#REGION .calendar .body{
    height: 16rem;
    overflow: hidden;
}

</style>

<div id="REGION" class="hidden">
	<div class="startTimeHolder"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion = this;
			var startDt = curRegion.paraMap.get("startDate");
			var endDt = curRegion.paraMap.get("endDate");
			var minDate = curRegion.paraMap.get("minDate");
			var maxDate = curRegion.paraMap.get("maxDate");
			var exclude = curRegion.paraMap.get("exclude");
			
			var calStyle = curRegion.paraMap.get("calStyle");
			if(calStyle==null)calStyle = 1;
			
			if(startDt=="")startDt=null;
			if(endDt=="")endDt=null;
			var dateFormat = curRegion.paraMap.get("dateFormat");
			if(dateFormat==null)dateFormat = "YYYY-MM-DD";
		
			var startDtMoment = null;
			if(startDt!=null){
				startDtMoment = moment(startDt,dateFormat);
			}
			
			var endDtMoment = null;
			if(endDt!=null){
				endDtMoment = moment(endDt,dateFormat);
			}
			
			var dateCoordMoment = null;
			if(startDtMoment!=null&& startDtMoment.isValid()){
				dateCoordMoment = startDtMoment;
			}
			else{
				dateCoordMoment = moment();
			}

			var paraMap1 = new HashMap();
			paraMap1.put("dateFormat",dateFormat);
			paraMap1.put("dateCoord",dateCoordMoment.format(dateFormat));
			paraMap1.put("startDate",startDt);
			paraMap1.put("endDate",endDt);
			paraMap1.put("minDate",minDate);
			paraMap1.put("maxDate",maxDate);
			paraMap1.put("calStyle",calStyle);
			if(exclude!=null)paraMap1.put("exclude",exclude);
			
			
			var startLoadPromise = RS.loadRegion(curRegion.find(".startTimeHolder"),RS.appendParaMapToUrl("common/shared/date/core/rangedate.rs",paraMap1));
			startLoadPromise.done(function(startDateRegion){
				curRegion.startDateRegion = startDateRegion;
				
				startDateRegion.onDayClick=function(dayCell,cellDate){
					REGION.handleRangeClick(curRegion,dayCell,cellDate);
				}
			})
		},
		handleRangeClick:function(curRegion,dayCell,cellDate){
			var startDateRegion = curRegion.startDateRegion;

			if(curRegion.selecting == false){
				curRegion.selecting = true;
				startDateRegion.startDate = cellDate;
				startDateRegion.endDate = null;
				startDateRegion.renderCalendar();
				
				startDateRegion.onDayHover = function(dayCell,date){
					startDateRegion.endDate = date;
					
					startDateRegion.selectRange(startDateRegion.startDate,startDateRegion.endDate);
				};
				if(typeof curRegion.onRangeSelectStart==="function"){
					curRegion.onRangeSelectStart(curRegion.getDate());
				}
			}
			else{
				delete startDateRegion.onDayHover;
				
				startDateRegion.endDate = cellDate;
				
				startDateRegion.renderCalendar();
				curRegion.selecting = false;
				
				if(typeof curRegion.onRangeSelectEnd==="function"){
					curRegion.onRangeSelectEnd(curRegion.getDate());
				}
			}
		},
		getDate:function(){//获取选中的日期
			var result = {};
			if(this.startDateRegion.startDate!=null)
				result.startDate = this.startDateRegion.startDate.format(this.startDateRegion.dateFormat);
			
			if(this.startDateRegion.endDate!=null)
				result.endDate = this.startDateRegion.endDate.format(this.startDateRegion.dateFormat);
			
			if(result.startDate>result.endDate){
				var tmp = result.startDate;
				result.startDate = result.endDate;
				result.endDate = tmp;
			}
			
			return result;
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.selecting = false;//正在选择日期
	staticRegion.getDate = REGION.getDate;
	staticRegion.onRangeSelectStart = null;//范围选择开始
	staticRegion.onRangeSelectEnd = null;//范围选择完毕
	staticRegion.renderRegion();
})
</script>