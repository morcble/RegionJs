var RegionTagPlugin = {//TODO formp.ackdata
		render:function(region,tmpObj,paraData){
			var resType = tmpObj.attr("resType");
			var width = tmpObj.attr("width");
			var height = tmpObj.attr("height");
			
			var regionUuid = tmpObj.attr("region-uuid");
			var parentRegionId = tmpObj.attr("region-id");
			var innerRegionHolder =$('<div class="inner-region" region-id="'+parentRegionId+'"></div>');
			
			if(height==null)height="100%";
			if(width==null)width="100%";
			innerRegionHolder.css("height",height);
			innerRegionHolder.css("width",width);
			
			var innerRegionId = null;
			if(regionUuid==null){
				regionUuid = RegionUtil.getUUID();				
			} 

			innerRegionId = parentRegionId+"_"+regionUuid;
		
		
			var regionAttr = tmpObj.attr("region-attr");	
			innerRegionHolder.attr("region-attr",regionAttr);
			//if(tmpObj.hasClass("region-editable"))innerRegionHolder.addClass("region-editable");
			if(resType==null)resType="relativeUrl";

			var res = tmpObj.attr("res");
			if(resType=="html"&&res!=null){
				var paraMap = null;
				res = res.replaceAll("region_"+parentRegionId,"REGION");
				RegionUtil.loadRegionByContent(innerRegionHolder,null,res,paraMap,innerRegionId);
			}
			else if(resType=="json"){
				var paraMap = null;
				RegionUtil.loadRegionByConfig(innerRegionHolder,JSON.parse(res),paraMap,innerRegionId);
			}
			else if(resType=="relativeUrl"){
				RegionUtil.loadRegion(innerRegionHolder,RegionUtil.appendParaObjectToUrl(res,paraData),innerRegionId);
			}
			else if(resType=="absoluteUrl"){
				RegionUtil.loadAbsRegion(innerRegionHolder,RegionUtil.appendParaObjectToUrl(res,paraData),innerRegionId);
			}
			else if(resType=="repo"){//中央仓库
				if(Config.rsRepo==null){
					Config.rsRepo = "http://cdn.rcsdn.cn/rsrepo/";
				}
				RegionUtil.loadAbsRegion(innerRegionHolder,RegionUtil.appendParaObjectToUrl(Config.rsRepo+res,paraData),innerRegionId);
			}

			tmpObj.replaceWith(innerRegionHolder);
			
			
			return innerRegionHolder;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			
		},
		packData:function(tmpObj){
			var innerRegionId = tmpObj.children()[0].getAttribute("region-id");
			var innerRegion = RegionUtil.getRegionById(innerRegionId);
			if(innerRegion==null)return null;
			if(innerRegion.type == "RegionForm"){
				var innerData = innerRegion.packFormData();
				return innerData;
			}
			else return null;
		}
}