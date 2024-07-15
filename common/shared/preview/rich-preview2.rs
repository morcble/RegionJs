<!-- 例子
var paraMap = new HashMap();
paraMap.put("url",Config.backendPath+"/RCS_SZPT/szappconfig/get");
paraMap.put("id",rowData.id);
RegionUtil.openModalWindow(RegionUtil.appendParaMapToUrl("common/shared/preview/rich-preview2.rs",paraMap),"简介",2,0.8,0.7); 
-->
<div id="REGION" class="hidden" style="padding-top:1rem;">
	<iframe class="holder" scrolling="no" style="width: 100%;border: 0;"></iframe>
</div>


<script type="text/javascript">
var REGION = {
		afterRenderData:function(){
			var curRegion=this;
			curRegion.find(".holder").attr("src",RegionUtil.appendParaMapToUrl("common/shared/preview/rich-preview2.html",curRegion.paraMap));
			curRegion.find(".holder").load(function(){   
				 	var iframeWindow = this;
					var doc = iframeWindow.contentDocument;
					var html = doc.documentElement;
					var body = doc.body;
					var height = Math.max( body.scrollHeight, body.offsetHeight,html.clientHeight, html.scrollHeight, html.offsetHeight );
					iframeWindow.setAttribute('height', height);
			});
			
			//TODO 位置错误，是否放到load里面，页面大小还需要重新适应
			var paraMap = curRegion.paraMap;
			var url=paraMap.remove("url");
			var reqObj = {};
			var keys = paraMap.keySet();
			for(var i = 0 ; i <keys.length ;i++){
				reqObj[keys[i]] = paraMap.get(keys[i]);
			}
			RegionUtil.loadingStart();
			RegionUtil.call(url,"POST",reqObj,function(response){
				RegionUtil.loadingComplete();
				if(response.success){
					let win = curRegion.find(".holder")[0].contentWindow;
					let doc = win.document;
					let html  = doc.getElementsByTagName('html')[0];
					html.innerHTML = response.data.description;
				}
				else{
					RegionUtil.alert(response.msg);
				}
				
			})
		}
};


RegionUtil.ready(function(){
	var staticRegion = RegionUtil.newStaticRegion("#REGION",null);
	staticRegion.afterRenderData = REGION.afterRenderData;
	staticRegion.renderRegion();
})
</script>