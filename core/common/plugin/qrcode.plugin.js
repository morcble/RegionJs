var QrcodePlugin = {
		render:function(region,tmpObj,paraData,indexNo){
			tmpObj.attr("type","hidden");
			tmpObj.addClass("wrapped");
			tmpObj.addClass("hidden");
			var format = tmpObj.attr("format");
			var regionAttr = tmpObj.attr("region-attr");
			var qrcodeId = "qrcode_"+region.regionId +"_" + regionAttr;
			if(indexNo!=null)qrcodeId+=indexNo;//list列表序号
			
			tmpObj.attr("qrcode-id",qrcodeId);
			var hiddenInputHtml = tmpObj.prop("outerHTML");
			
			var height = tmpObj.attr("height");
			var width = tmpObj.attr("width");
			
			if(height==null)height="100";
			else height = height.replace("px","");
			
			if(width==null)width="100";
			else width = width.replace("px","");
			
			var colorDark = tmpObj.attr("color-dark");
			var colorLight = tmpObj.attr("color-light");
			var correctLevel = tmpObj.attr("correct-level");
			if(colorDark==null||colorDark=="")colorDark="#000000";
			if(colorLight==null||colorLight=="")colorLight="#ffffff";
			if(correctLevel==null||correctLevel=="")correctLevel="H";
			
			var replaceHtml = '<div class="qrcode-wrapper region-wrapper">'+hiddenInputHtml;
			replaceHtml += '<div id="'+qrcodeId+'"></div>';
			replaceHtml+='</div>';
			
			
			var replaced = $(replaceHtml);			
			RegionUtil.addRegionUniqueId(replaced[0],region.regionId);
			tmpObj.replaceWith(replaced);
			
			new QRCode(qrcodeId, {
			    text: paraData,
			    width: width,
			    height: height,
			    colorDark : colorDark,
			    colorLight : colorLight,
			    correctLevel : QRCode.CorrectLevel[correctLevel]//H L M Q
			});
			
			tmpObj = replaced.find(".wrapped");
			tmpObj.attr("real-val",paraData);			
			return tmpObj;
		},
		updateVal:function(region,wrapperElemObj,regionElemObj,paraData){//值改变调用此函数
			var format = regionElemObj.attr("format");
			var height = regionElemObj.attr("height");
			var width = regionElemObj.attr("width");
			
			if(height==null)height="100";
			else height = height.replace("px","");
			
			if(width==null)width="100";
			else width = width.replace("px","");
			
			var colorDark = regionElemObj.attr("color-dark");
			var colorLight = regionElemObj.attr("color-light");
			var correctLevel = regionElemObj.attr("correct-level");
			if(colorDark==null||colorDark=="")colorDark="#000000";
			if(colorLight==null||colorLight=="")colorLight="#ffffff";
			if(correctLevel==null||correctLevel=="")correctLevel="H";
			
			var qrcodeId = regionElemObj.attr("qrcode-id");
			
			var replaceHtml = regionElemObj.prop("outerHTML");
			replaceHtml += '<div id="'+qrcodeId+'"></div>';
			wrapperElemObj.html(replaceHtml);
			
			new QRCode(qrcodeId, {
			    text: paraData,
			    width: width,
			    height: height,
			    colorDark : colorDark,
			    colorLight : colorLight,
			    correctLevel : QRCode.CorrectLevel[correctLevel]//H L M Q
			});
			wrapperElemObj.find(".wrapped").attr("real-val",paraData);
		}
}