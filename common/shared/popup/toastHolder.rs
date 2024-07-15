<style type="text/css">
	#REGION{
		pointer-events:none;
	}
	
	#REGION>.card{
		box-shadow: 0 3px 6px -4px #0000001f, 0 6px 8px #00000014, 0 4px 4px 4px #0000000d;
		pointer-events:all;
		position:relative;
		
		margin-bottom: 1rem;
	}
	
	#REGION .fa-circle-info{
        color: #1663f1;
    }

   
	
</style>

<div id="REGION" class="hidden">
	<div class="brace"></div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			//var paraName = this.paraMap.get("paraName");
			
		},
		toast:function(rsUrl){
			var curRegion = this;
			var cardId = "card"+RS.getUUID();
			var toastCard = "<div cardId='"+cardId+"' class='card "+cardId+" movein-animation1-class'></div>";
			RS.insertAfter(curRegion,curRegion.find(".brace")[0],toastCard);
			return RS.loadRegion(curRegion.find("."+cardId),rsUrl,cardId);
		},
		closeCard:function(cardId){
			var curRegion = this;
			var cardDiv = curRegion.find("."+cardId);
			if(cardDiv.length==0)return;
			cardDiv.bind('webkitAnimationEnd mozAnimationEnd MSAnimationEnd oanimationend animationend', function(){
				RS.getRegionById($(this).attr("cardId")).release();
				$(this).remove();
			});
			cardDiv.removeClass("movein-animation1-class").addClass("moveout-animation1-class");
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.toast = REGION.toast;
	staticRegion.closeCard = REGION.closeCard;
	staticRegion.renderRegion();
})
</script>