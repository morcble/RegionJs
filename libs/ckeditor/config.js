/**
 * @license Copyright (c) 2003-2019, CKSource - Frederico Knabben. All rights reserved.
 * For licensing, see https://ckeditor.com/legal/ckeditor-oss-license
 */

CKEDITOR.editorConfig = function( config ) {
	// Define changes to default configuration here. For example:
	// config.language = 'fr';
	// config.uiColor = '#AADC6E';
	config.height = 150;
	config.width = '100%';
	config.toolbarGroups = [
		{ name: 'document', groups: [ 'document', 'mode', 'doctools' ] },
		{ name: 'insert', groups: [ 'insert' ] },
		{ name: 'basicstyles', groups: [ 'basicstyles', 'cleanup' ] },
		{ name: 'paragraph', groups: [ 'list', 'indent', 'blocks', 'align', 'bidi', 'paragraph' ] },
		{ name: 'clipboard', groups: [ 'clipboard', 'undo' ] },
		{ name: 'editing', groups: [ 'find', 'selection', 'spellchecker', 'editing' ] },
		{ name: 'forms', groups: [ 'forms' ] },
		{ name: 'links', groups: [ 'links' ] },
		{ name: 'styles', groups: [ 'styles' ] },
		{ name: 'colors', groups: [ 'colors' ] },
		{ name: 'tools', groups: [ 'tools' ] },
		{ name: 'others', groups: [ 'others' ] },
		{ name: 'about', groups: [ 'about' ] },
		
	];

	//config.removeButtons = 'Save,Templates,NewPage,Flash,HorizontalRule,Smiley,PageBreak,Iframe,CreateDiv,Outdent,Indent,BidiLtr,BidiRtl,Language,Undo,Redo,Cut,Copy,Paste,PasteText,PasteFromWord,Find,Replace,SelectAll,Scayt,Form,Checkbox,Radio,TextField,Textarea,Select,Button,ImageButton,HiddenField,Anchor,Unlink,ShowBlocks,About,Font,Styles,Print';
	config.removeButtons = 'Save,Templates,NewPage,Flash,Smiley,Iframe,CreateDiv,Outdent,Indent,BidiLtr,BidiRtl,Language,Undo,Redo,Paste,PasteText,Replace,SelectAll,Scayt,TextField,Select,Button,ImageButton,HiddenField,About';
	
	config.extraPlugins="video";
	//config.extraPlugins="latex,codesnippet";
    //使用zenburn的代码高亮风格样式 PS:zenburn效果就是黑色背景
    //如果不设置着默认风格为default
   // config.codeSnippet_theme =  'zenburn';
	config.enterMode = CKEDITOR.ENTER_BR;
	config.shiftEnterMode = CKEDITOR.ENTER_P;
	config.allowedContent = true;//允许插入别的dom 标签
	config.language = 'zh-cn'; 
	config.font_names = '宋体/SimSun;新宋体/NSimSun;仿宋/FangSong;楷体/KaiTi;仿宋_GB2312/FangSong_GB2312;'+  
    '楷体_GB2312/KaiTi_GB2312;黑体/SimHei;华文细黑/STXihei;华文楷体/STKaiti;华文宋体/STSong;华文中宋/STZhongsong;'+  
    '华文仿宋/STFangsong;华文彩云/STCaiyun;华文琥珀/STHupo;华文隶书/STLiti;华文行楷/STXingkai;华文新魏/STXinwei;'+  
    '方正舒体/FZShuTi;方正姚体/FZYaoti;细明体/MingLiU;新细明体/PMingLiU;微软雅黑/Microsoft YaHei;微软正黑/Microsoft JhengHei;'+  
    'Arial Black/Arial Black;'+ config.font_names;  
	
	config.mathJaxLib = "https://cdn.bootcss.com/mathjax/2.7.6/MathJax.js?config=TeX-AMS_HTML&amp;t=J8Q8";//ResFilePrefix+"libs/mathjax/2.7.6/MathJax.js?config=TeX-AMS_HTML&amp;t=J8Q8";
	//https://cdn.bootcss.com/mathjax/2.7.6/MathJax.js
};


//增加视频上传插件,借用image标签
CKEDITOR.on('instanceReady', function(ev) {
    ev.editor.on('doubleclick', function(evt){
        var element = evt.data.element;
        var editor = evt.editor;
        var dialogType = evt.data.dialog;
        if("image"==dialogType){
        	var targetTag = evt.data.element.$;
        	
        	var isVideo = targetTag.classList.contains('video');
        	if(isVideo){
        		evt.stop();
        	} 
        }
    });
});
