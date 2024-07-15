<style>
.vertical_center{
  position: relative;
  top:50%;
  transform:translateY(-50%);
  max-height:100%;
  max-width:100%;
}

#REGION .swiper-slide{
	text-align: center;
}

</style>

<div id="REGION" class="hidden" style="background: darkslategray;padding:0rem; overflow: auto;">
	<div style="height: 100%;position: absolute;right: 0rem;top:0rem">
		 <span class="fa-solid fa-remove hidden" aria-hidden="true" onclick="RegionUtil.gotoBack()" style="z-index:100;color:white;-webkit-text-stroke:4px darkslategray"></span>
	</div>
	<div class="preview-holder" style="height: 100%;">
		<div class="REGIONSwiper swiper-container" style="height: 100%;">
            <div class="swiper-wrapper imgOrVideos" ></div>
        	
        	<div class="swiper-pagination"></div>
        </div>
	</div>
</div>	


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var hideClose = this.paraMap.get("hideClose");
			if("true"!=hideClose){
				this.find(".fa-remove").removeClass("hidden")
			}
			
			
			var imgsJson = this.paraMap.get("imgsJson");
			var videosJson = this.paraMap.get("videosJson");
			if((imgsJson==null||imgsJson=="")&&(videosJson==null||videosJson==""))return;
			
			var imgs = JSON.parse(imgsJson);
			var videos = JSON.parse(videosJson);
			console.log(imgs)
			console.log(videos)
			
			var htmlStr = "";
			if(imgs!=null){
				for(var i = 0 ; i <imgs.length ;i++){
					htmlStr+='<div class="swiper-slide" > ';
					htmlStr+='	<div class="swiper-zoom-container">';
		            htmlStr+='   		<img class="" src="'+imgs[i]+'">  ';
		            htmlStr+='	</div> ';
		            htmlStr+='</div> ';
				}
			}
			
			if(videos!=null){
				for(var i = 0 ; i <videos.length ;i++){
					htmlStr+='<div class="swiper-slide" > ';
		            htmlStr+='   		<video class="vertical_center" src="'+videos[i]+'" alt="" controls="controls" style="max-width: 100%;"></video>  ';
		            htmlStr+='</div> ';
				}
			}
			
			
			
			var curRegion = this;
			RegionUtil.appendHtml(curRegion,curRegion.find(".imgOrVideos")[0],htmlStr);
			
	
			var swiper = new Swiper('.REGIONSwiper', {
				    zoom: true,
		            loop: false,
		            autoplay:false,
		            observer:true,
		            observeParents:true,
		            pagination: {
		                el: '.swiper-pagination',
		                clickable :true
		                }
		            });
			
			console.log(curRegion.find('.swiper-slide'))
			
			/* setTimeout(function(){
				curRegion.find('img').each(function(){
					RegionUtil.enableScale(this,$(this).closest(".swiper-slide")[0]);
				})
			}) */
			
			
			/* curRegion.find('.vertical_center').bind('touchstart',function(event){
				event.preventDefault();
				alert("left:"+event.clientX+",top:"+event.clientY)
				
			}); */
			
			//curRegion.find('.vertical_center').bind('swiperight',REGION.manipulate);
			//curRegion.find('.vertical_center').bind('swiperleft',REGION.manipulate);
			//urRegion.find('.vertical_center').bind('pinchopen',REGION.manipulate);
			//curRegion.find('.vertical_center').bind('swipeup',REGION.manipulate);
			//curRegion.find('.vertical_center').bind('pinchclose',REGION.manipulate);
			
			//this.find(".preview").attr("src",imgBase64);
		}
		
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>