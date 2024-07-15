var CodePlugin = {
		render:function(region,tmpObj,paraData){
			tmpObj.attr("type","hidden");
			tmpObj.addClass("wrapped");
			tmpObj.addClass("hidden");
			
			var regionAttr = tmpObj.attr("region-attr");
			
			var replaceHtml = '<div class="code-wrapper region-wrapper">';
			replaceHtml += tmpObj.prop("outerHTML");
			replaceHtml += '<code></code>';
			replaceHtml+='</div>';
			
			var replaced = $(replaceHtml);
			
			var codeTag = replaced.find("code");
			codeTag.attr("width",tmpObj.attr("width"));
			codeTag.attr("height",tmpObj.attr("height"));
			codeTag.attr("name",tmpObj.attr("name"));
			codeTag.attr("showheader",tmpObj.attr("showheader"));
			codeTag.attr("escapeHtml",tmpObj.attr("escapeHtml"));
			
			replaced.css("min-width",tmpObj.attr("width"));
			
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			codeTag.text(paraData);
			
			replaced.find(".wrapped").attr("real-val",paraData);
			
			RegionUtil.resolveSingleCode(region,codeTag);
			return replaced;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			var replaceHtml = '<code></code>';
			var replaced = $(replaceHtml);

			replaced.attr("width",regionElemObj.attr("width"));
			replaced.attr("height",regionElemObj.attr("height"));
			replaced.attr("name",regionElemObj.attr("name"));
			replaced.attr("showheader",regionElemObj.attr("showheader"));
			replaced.attr("escapeHtml",regionElemObj.attr("escapeHtml"));
			
			wrapperElemObj.css("min-width",regionElemObj.attr("width"));
			
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			wrapperElemObj.find(".codepre").replaceWith(replaced);
			var codeTag = wrapperElemObj.find("code");
			codeTag.text(paraData);
			RegionUtil.resolveSingleCode(region,codeTag);
			
			wrapperElemObj.find(".wrapped").attr("real-val",paraData);
		}
}