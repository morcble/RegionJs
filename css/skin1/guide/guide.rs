<style type="text/css">
    #REGION {
        display: flex;
        justify-content: space-between;
        padding-bottom: 0;

    }

    #REGION .left {
        width: 18rem;
        float: left;
        box-sizing: border-box;
        height: 100%;
    }

    #REGION .left span {
        display: block;
        font-size: 1.25rem;
        line-height: 4rem;
        cursor: pointer;
    }

    #REGION .right {
        width: calc(100% - 21rem);
        float: right;
        height: 100%;
        overflow-y: auto;
        padding-bottom: 2rem;
    }

    #REGION a {
        cursor: pointer;
    }
    #REGION a:hover {
        text-decoration: none;
    }

    #REGION h3 {
        padding: 1rem 0;
    }

    #REGION .width-flex {
        float: left;
        width: 100%;
    }

    #REGION .height-button {
        width: 100%;
        padding: 1rem 0;
    }

    #REGION .title-shuo, .title-form {
        width: 100%;
        font-size: 1.25rem;
        line-height: 2rem;
        box-sizing: border-box;
        position: relative;
    }

    #REGION .tab-box {
        width: 100%;
    }

    #REGION .tab-box .box-ul {
        width: 100%;
        display: flex;
    }

    #REGION .tab-box .box-ul li {
        padding: 0.125rem 0.375rem;
        position: relative;
    }

    #REGION .se {
        width: 32rem;
        display: flex;
        justify-content: flex-start;
        align-items: center;
    }

    #REGION .se span {
        width: calc(100% / 4);
        display: block;
        text-align: center;
        cursor: pointer;
    }

    #REGION .color-blue {
        color: #0D34FF;
    }

    #REGION .title-table {
        position: relative;
    }
	#REGION .title-theme{
		position: relative;
        width: 100%;
        height: 62rem;
        overflow: hidden;
	}

</style>

<div id="REGION" class="hidden background-ff" no-footer>
    <div class="left">
        <span class="guides-default guides-active" onclick="REGION.goSection(event,1)">Button按钮</span>
        <span class="guides-default" onclick="REGION.goSection(event,2)">form表单的用法</span>
        <span class="guides-default" onclick="REGION.goSection(event,3)">table表格</span>
        <span class="guides-default" onclick="REGION.goSection(event,4)">内容溢出省略表示</span>
        <span class="guides-default" onclick="REGION.goSection(event,5)">主题色切换</span>
        <span class="guides-default" onclick="REGION.goSection(event,6)">页面结构组成</span>
    </div>
    <div class="right">
        <div class="width-flex">
            <h3 id="REGION_dv1">1.Button按钮</h3>
            <h4>常用的操作按钮。</h4>
            <div class="height-button">
                <button class="btn" data-type="default" normal>默认按钮</button>
                <button class="btn" data-type="primary" normal>主要按钮</button>
                <button class="btn" data-type="success" normal>成功按钮</button>
                <button class="btn" data-type="info" normal>信息按钮</button>
                <button class="btn" data-type="warning" normal>警告按钮</button>
                <button class="btn" data-type="danger" normal>危险按钮</button>
                <div style="height: 10px;width: 100%"></div>
                <button class="btn" data-type="default" plain>朴素按钮</button>
                <button class="btn" data-type="primary" plain>主要按钮</button>
                <button class="btn" data-type="success" plain>成功按钮</button>
                <button class="btn" data-type="info" plain>信息按钮</button>
                <button class="btn" data-type="warning" plain>警告按钮</button>
                <button class="btn" data-type="danger" plain>危险按钮</button>
                <div style="height: 10px;width: 100%"></div>
                <button class="btn" data-type="default" normal round>圆角按钮</button>
                <button class="btn" data-type="primary" normal round>主要按钮</button>
                <button class="btn" data-type="success" normal round>成功按钮</button>
                <button class="btn" data-type="info" normal round>信息按钮</button>
                <button class="btn" data-type="warning" normal round>警告按钮</button>
                <button class="btn" data-type="danger" normal round>危险按钮</button>
                <div style="height: 10px;width: 100%"></div>
                <button data-type="default" circle><i class="fa-solid fa-search"></i></button>
                <button data-type="primary" circle><i class="fa-solid fa-search"></i></button>
                <button data-type="success" circle><i class="fa-solid fa-search"></i></button>
                <button data-type="info" circle><i class="fa-solid fa-search"></i></button>
                <button data-type="warning" circle><i class="fa-solid fa-search"></i></button>
                <button data-type="danger" circle><i class="fa-solid fa-search"></i></button>

            </div>
            <div class="title-shuo">
                <p>使用data-type、plain、normal、round和circle属性来定义 Button 的样式。</p>
                <p>data-type类型有[default,primary,info,warning,danger,success]
                <div class="col-xs-12">
                    <code showheader="true" minimized="false">
 &lt;button class=&quot;btn&quot; data-type=&quot;default&quot; normal&gt;默认按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;primary&quot; normal &gt;主要按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;success&quot; normal &gt;成功按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;info&quot; normal &gt;信息按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;warning&quot; normal &gt;警告按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;danger&quot; normal &gt;危险按钮&lt;/button&gt;

 &lt;button class=&quot;btn&quot; data-type=&quot;default&quot; plain&gt;朴素按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;primary&quot; plain &gt;主要按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;success&quot; plain &gt;成功按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;info&quot; plain &gt;信息按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;warning&quot; plain &gt;警告按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;danger&quot; plain &gt;危险按钮&lt;/button&gt;

 &lt;button class=&quot;btn&quot; data-type=&quot;default&quot; normal round&gt;圆角按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;primary&quot; normal round&gt;主要按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;success&quot; normal round&gt;成功按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;info&quot; normal round&gt;信息按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;warning&quot; normal round&gt;警告按钮&lt;/button&gt;
 &lt;button class=&quot;btn&quot; data-type=&quot;danger&quot; normal round&gt;危险按钮&lt;/button&gt;

 &lt;button data-type=&quot;default&quot; circle&gt;&lt;i class=&quot;fa-solid fa-search&quot;&gt;&lt;/i&gt;&lt;/button&gt;
 &lt;button data-type=&quot;primary&quot; circle&gt;&lt;i class=&quot;fa-solid fa-search&quot;&gt;&lt;/i&gt;&lt;/button&gt;
 &lt;button data-type=&quot;success&quot; circle&gt;&lt;i class=&quot;fa-solid fa-search&quot;&gt;&lt;/i&gt;&lt;/button&gt;
 &lt;button data-type=&quot;info&quot; circle&gt;&lt;i class=&quot;fa-solid fa-search&quot;&gt;&lt;/i&gt;&lt;/button&gt;
 &lt;button data-type=&quot;warning&quot; circle&gt;&lt;i class=&quot;fa-solid fa-search&quot;&gt;&lt;/i&gt;&lt;/button&gt;
 &lt;button data-type=&quot;danger&quot; circle&gt;&lt;i class=&quot;fa-solid fa-search&quot;&gt;&lt;/i&gt;&lt;/button&gt;
                    </code>
                </div>
            </div>

        </div>
        <div class="width-flex">
            <h3 id="REGION_dv2">2.form表单的用法</h3>
            <div class="title-shuo">
                <p>在类名form-body 内进行 现有表单样式 结构解析</p>
                <p>通过设置 label-position 属性可以改变表单域标签的位置，可选值为 top、left，当设为 top 时标签会置于表单域的顶部;</p>
                <p>可以设置 border 属性改变 表单样式 (当设置border属性时 label-position=top 失效)</p>
                <p>is-required 为必填属性class</p>
                <p>如果觉得默认显示9个字过宽，直接在当前页面进行覆盖设置你想要的宽度:</p>
                <p> #REGION .form-body .form-item-public > .form-label{
                    width:5rem;
                    }
                    <br>
                    #REGION .form-body .form-item-public > .form-label ~ .error-msg{
                    left:5rem;
                    }(若没有必填可忽略)
                </p>
                <p>保存按钮位置设置,默认排列在表单下方,添加.footer 类名,表示固定浮动在底部</p>
            </div>
            <div class="title-form">

            </div>
            <div class="col-xs-12">
                <code showheader="true" minimized="false">
 &lt;div class=&quot;form-body&quot; &gt;
    &lt;div class=&quot;col-xs-12 col-md-12 form-item-public&quot;&gt;
         &lt;message key=&quot;默认显示&quot; class=&quot;form-label&quot;&gt;&lt;/message&gt;
          &lt;input type=&quot;text&quot; region-attr=&quot;&quot; class=&quot;form-control&quot;&gt;
    &lt;/div&gt;
    &lt;div class=&quot;col-xs-12 col-md-12 form-item-public is-required&quot;&gt;
         &lt;message key=&quot;默认显示&quot; class=&quot;form-label&quot;&gt;&lt;/message&gt;
         &lt;input type=&quot;text&quot; region-attr=&quot;&quot; class=&quot;form-control&quot;&gt;
    &lt;/div&gt;
    &lt;div class=&quot;col-xs-12 col-md-12 form-item-public is-required&quot;&gt;
         &lt;message key=&quot;默认显示&quot; class=&quot;form-label&quot;&gt;&lt;/message&gt;
         &lt;input type=&quot;text&quot; region-attr=&quot;&quot; class=&quot;form-control&quot;&gt;
    &lt;/div&gt;
    &lt;div class=&quot;col-xs-12 col-md-12 form-item-public is-required&quot;&gt;
          &lt;message key=&quot;默认最多显示9个字&quot; class=&quot;form-label&quot;&gt;&lt;/message&gt;
          &lt;input type=&quot;text&quot; region-attr=&quot;&quot; class=&quot;form-control&quot;&gt;
    &lt;/div&gt;
    &lt;div class=&quot;col-xs-12 col-md-12 form-item-public is-required&quot;&gt;
          &lt;message key=&quot;默认最多显示9个字&quot; class=&quot;form-label&quot;&gt;&lt;/message&gt;
          &lt;input type=&quot;text&quot; region-attr=&quot;&quot; class=&quot;form-control&quot;&gt;
   &lt;/div&gt;
   &lt;div class=&quot;col-xs-12 col-md-12 text-center&quot;&gt;
          &lt;button class=&quot;btn&quot; data-type=&quot;primary&quot; normal &gt;保存&lt;/button&gt;
          &lt;button class=&quot;btn&quot; data-type=&quot;default&quot; normal&gt;重置&lt;/button&gt;
    &lt;/div&gt;
 &lt;/div&gt;
                </code>
            </div>

	    </div>
	    <div style="width: 100%; margin-top:20px;float: left">
	        <div style="float: left;width:100%;">
	            <h3 id="REGION_dv3">3.table表格的用法</h3>
	            <div class="title-shuo">
	                <p>通过设置 label-position 属性可以改变表单域标签的位置，可选值为center、left,right;</p>
	                <p>可以设置 border 属性改变 表格样式</p>
	                <p>设置 no-index 属性 是去掉第一列中有默认padding-left:60px的值</p>
	                <p>设置 第一列为固定宽度 类名maxwidth60(通常第一列为序号时使用) 时  例如:<span><</span><span>div class="col-xs-1 maxwidth60"></span><span><</span><span>/div></span>
	                    当前 <span><</span><span>div class="table-body">中需要去掉 no-index属性</span>
	                </p>
	                <h3>用法1：常规写法【 文字提示('prompt' 属性 常用于表格文字过长展示鼠标 hover 时的提示全部信息),'copy'属性 用于复制当前文本】</h3>
	            </div>
	            <div class="title-table"></div>
				<div class="col-xs-12">
					<code showheader="true" minimized="fase">
&lt;div class=&quot;table-body&quot;  border no-index&gt;
    &lt;div class=&quot;datas-columns row&quot;&gt;
        &lt;div class=&quot;col-xs-2&quot;&gt;&lt;message key=&quot;name&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-1&quot;&gt;&lt;message key=&quot;status&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-2&quot;&gt;&lt;message key=&quot;global_msg.create_dt&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-4&quot;&gt;&lt;message key=&quot;description&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-3 text-center&quot;&gt;&lt;message key=&quot;global_msg.operation&quot;&gt;&lt;/message&gt;&lt;/div&gt;
    &lt;/div&gt;
    &lt;div class=&quot;datas&quot; region-list=&quot;list&quot;&gt;
        &lt;div class=&quot;row&quot;&gt;
            &lt;div class=&quot;col-xs-2&quot; copy&gt;&lt;span region-attr=&quot;name&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-1&quot;&gt;&lt;span region-attr=&quot;status&quot; region-ds=&quot;EnableStatus&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-2&quot;&gt;&lt;span region-attr=&quot;shortCreateDt&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-4&quot; prompt&gt;&lt;span region-attr=&quot;description&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-3 text-center&quot;&gt;
                &lt;button class=&quot;btn&quot;  data-type=&quot;primary&quot; plain&gt;解析&lt;/button&gt;
                &lt;button class=&quot;btn&quot; data-type=&quot;primary&quot; plain&gt;&lt;i class=&quot;fa-light fa-pencil&quot; title=&quot;编辑&quot;&gt;&lt;/i&gt;&lt;/button&gt;
                &lt;button class=&quot;btn&quot;   data-type=&quot;danger&quot; plain&gt;删除&lt;/button&gt;
            &lt;/div&gt;
        &lt;/div&gt;
    &lt;/div&gt;
&lt;/div&gt;


					</code>
				</div>
					<h3 style="padding: 1rem 0;width: 100%;float: left">用法2:添加序号 默认文字高亮显示(active)</h3>
				<div class="title-table1"></div>
				<div class="col-xs-12">
					<code showheader="true" minimized="false">
&lt;div class=&quot;table-body&quot;   border&gt;
	&lt;div class=&quot;datas-columns row&quot;&gt;
		&lt;div class=&quot;col-xs-1 maxwidth60&quot;&gt;&lt;message key=&quot;global_msg.index&quot;&gt;&lt;/message&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-2&quot;&gt;&lt;message key=&quot;name&quot;&gt;&lt;/message&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-1&quot;&gt;&lt;message key=&quot;status&quot;&gt;&lt;/message&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-2&quot;&gt;&lt;message key=&quot;global_msg.create_dt&quot;&gt;&lt;/message&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-4&quot;&gt;&lt;message key=&quot;description&quot;&gt;&lt;/message&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-3 text-center&quot;&gt;&lt;message key=&quot;global_msg.operation&quot;&gt;&lt;/message&gt;&lt;/div&gt;
	&lt;/div&gt;
	&lt;div class=&quot;datas&quot; region-list=&quot;list&quot;&gt;
		&lt;div class=&quot;row&quot;&gt;
		&lt;div class=&quot;col-xs-1 maxwidth60&quot;&gt;&lt;span region-attr=&quot;index&quot;&gt;&lt;/span&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-2 active &quot;&gt;&lt;span region-attr=&quot;name&quot;&gt;&lt;/span&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-1&quot;&gt;&lt;span region-attr=&quot;status&quot; region-ds=&quot;EnableStatus&quot;&gt;&lt;/span&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-2&quot;&gt;&lt;span region-attr=&quot;shortCreateDt&quot;&gt;&lt;/span&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-4&quot;&gt;&lt;span region-attr=&quot;description&quot;&gt;&lt;/span&gt;&lt;/div&gt;
		&lt;div class=&quot;col-xs-3 text-center&quot;&gt;
			&lt;button class=&quot;btn&quot;  data-type=&quot;primary&quot; plain&gt;解析&lt;/button&gt;
			&lt;button class=&quot;btn&quot; data-type=&quot;primary&quot; plain&gt;&lt;i class=&quot;fa-light fa-pencil&quot; title=&quot;编辑&quot;&gt;&lt;/i&gt;&lt;/button&gt;
			&lt;button class=&quot;btn&quot;   data-type=&quot;danger&quot; plain&gt;删除&lt;/button&gt;
		&lt;/div&gt;
	&lt;/div&gt;
		&lt;/div&gt;
&lt;/div&gt;

					</code>
				</div>
				<h3 style="padding: 1rem 0;width: 100%;float: left">用法3:表格左右固定 点击高亮选中事件(highlight)</h3>
				<div class="title-table2"></div>
				<div class="col-xs-12">
					<code showheader="true" minimized="false">

&lt;div class=&quot;table-body&quot;   border&gt;
    &lt;div class=&quot;datas-columns row&quot;&gt;
        &lt;div class=&quot;col-xs-1 maxwidth60&quot;&gt;&lt;message key=&quot;global_msg.index&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-2&quot;&gt;&lt;message key=&quot;name&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-1&quot;&gt;&lt;message key=&quot;status&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-2&quot;&gt;&lt;message key=&quot;global_msg.create_dt&quot;&gt;&lt;/message&gt;&lt;/div&gt;
        &lt;div class=&quot;col-xs-4 no-scrollbar tableScrollRow&quot;&gt;
            &lt;div class=&quot;container&quot;&gt;
                &lt;div class=&quot;col-xs-6&quot;&gt;&lt;message key=&quot;description&quot;&gt;&lt;/message&gt;&lt;/div&gt;
                &lt;div class=&quot;col-xs-6&quot;&gt;&lt;message key=&quot;updateBy&quot;&gt;&lt;/message&gt;&lt;/div&gt;
            &lt;/div&gt;
        &lt;/div&gt;
        &lt;div class=&quot;col-xs-3 text-center&quot;&gt;&lt;message key=&quot;global_msg.operation&quot;&gt;&lt;/message&gt;&lt;/div&gt;
    &lt;/div&gt;
    &lt;div class=&quot;datas&quot; region-list=&quot;list&quot;&gt;
        &lt;div class=&quot;row&quot; onclick="RegionUtil.handleListData(event,REGION.choose)" &gt;
            &lt;div class=&quot;col-xs-1 maxwidth60&quot;&gt;&lt;span region-attr=&quot;index&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-2&quot;&gt;&lt;span region-attr=&quot;name&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-1&quot;&gt;&lt;span region-attr=&quot;status&quot; region-ds=&quot;EnableStatus&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-2&quot;&gt;&lt;span region-attr=&quot;shortCreateDt&quot;&gt;&lt;/span&gt;&lt;/div&gt;
            &lt;div class=&quot;col-xs-4 no-scrollbar tableScrollRow&quot;&gt;
                &lt;div class=&quot;container&quot;&gt;
                    &lt;div class=&quot;col-xs-6&quot;&gt;&lt;span region-attr=&quot;description&quot;&gt;&lt;/span&gt;&lt;/div&gt;
                    &lt;div class=&quot;col-xs-6&quot;&gt;&lt;span region-attr=&quot;updateBy&quot;&gt;&lt;/span&gt;&lt;/div&gt;
                &lt;/div&gt;
            &lt;/div&gt;
            &lt;div class=&quot;col-xs-3 text-center&quot;&gt;
                &lt;button class=&quot;btn&quot;  data-type=&quot;primary&quot; plain&gt;解析&lt;/button&gt;
                &lt;button class=&quot;btn&quot; data-type=&quot;primary&quot; plain&gt;&lt;i class=&quot;fa-light fa-pencil&quot; title=&quot;编辑&quot;&gt;&lt;/i&gt;&lt;/button&gt;
                &lt;button class=&quot;btn&quot;   data-type=&quot;danger&quot; plain&gt;删除&lt;/button&gt;
            &lt;/div&gt;
        &lt;/div&gt;
    &lt;/div&gt;
&lt;/div&gt;

&lt;script type=&quot;text/javascript&quot;&gt;
    var REGION = {
        list:[
            {
                createBy:&quot;2100&quot;,
                createDt:&quot;2024-02-01 09:27:39&quot;,
                createDtAsStr:&quot;2024-02-01 09:27:39&quot;,
                description:&quot;每个服务器都有对应的DNS&quot;,
                id:&quot;4758672318060822528&quot;,
                name:&quot;rs.servers&quot;,
                shortCreateDt:&quot;2024-02-01&quot;,
                softDelete: 0,
                spId :&quot;100&quot;,
                status : 1,
                updateBy: &quot;2100&quot;,
                updateDt:&quot;2024-02-01 09:27:39&quot;,
            },
            {
                createBy:&quot;2100&quot;,
                createDt:&quot;2024-02-01 09:27:39&quot;,
                createDtAsStr:&quot;2024-02-01 09:27:39&quot;,
                description:&quot;每个服务器都有对应的DNS&quot;,
                id:&quot;4758672318060822528&quot;,
                name:&quot;rs.servers&quot;,
                shortCreateDt:&quot;2024-02-01&quot;,
                softDelete: 0,
                spId :&quot;100&quot;,
                status : 1,
                updateBy: &quot;2100&quot;,
                updateDt:&quot;2024-02-01 09:27:39&quot;,
            }
        ],
        beforeRenderData: function () {
            var curRegion=this;
            curRegion.regionData.list=REGION.list
        },
        afterRenderData:function(){
            var curRegion=this;
            if(!curRegion.inited){
                curRegion.inited = true;

                RS.enableScroll(curRegion.find(&quot;.table-body&quot;));
            }
        },
        choose:function (rowData,targetElem){
            var rowObj = $(targetElem).closest(&quot;.row&quot;);
            rowObj.addClass(&quot;highlight&quot;).siblings().removeClass(&quot;highlight&quot;);
        }
    };
    RegionUtil.ready(function () {
        var res = {
            message:{
                dft:{
                    spId:&quot;spId&quot;,
                    name:&quot;域名&quot;,
                    description:&quot;描述&quot;,
                    status:&quot;状态&quot;,
                    midServerId:&quot;midServerId&quot;
                }
            },

        };
        var gridRegion = RS.newFormRegion(&quot;#REGION&quot;,res);;
        gridRegion.beforeRenderData = REGION.beforeRenderData;
        gridRegion.afterRenderData = REGION.afterRenderData;
        gridRegion.renderRegion();
    })
&lt;/script&gt;


					</code>
				</div>
			</div>
	    </div>
	        <div style="width: 100%; margin-top: 30px;float: left;font-size: 1.25rem">
	            <h3 id="REGION_dv4">文本内容溢出表示样式</h3>
	            <h4>ellipsis-line-1(1-3)表示当前内容显示几行</h4>
	            <div class="ellipsis-line-1" style="width: 200px;margin-bottom: 30px">表示当前内容显示1行表示当前内容显示1行</div>
	            <div class="ellipsis-line-2" style="width: 200px;margin-bottom: 30px">表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行</div>
	            <div class="ellipsis-line-3" style="width: 200px;margin-bottom: 30px">表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行</div>
	         <div class="col-xs-12">
				 <code showheader="true" minimized="false">
&lt;div class=&quot;ellipsis-line-1&quot; style=&quot;width: 200px;background: #fff;margin-bottom: 30px&quot;&gt;表示当前内容显示1行表示当前内容显示1行&lt;/div&gt;
&lt;div class=&quot;ellipsis-line-2&quot; style=&quot;width: 200px;background: #fff;margin-bottom: 30px&quot;&gt;表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行表示当前内容显示2行&lt;/div&gt;
&lt;div class=&quot;ellipsis-line-3&quot; style=&quot;width: 200px;background: #fff;margin-bottom: 30px&quot;&gt;表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行表示当前内容显示3行&lt;/div&gt;

				 </code>
	         </div>
	        </div>
	    <div style="width: 100%; margin-top: 30px;float: left;padding-bottom: 50px">
	        <h3 id="REGION_dv5">5.主题色切换</h3>
	        <h4>当下流行定制主题颜色,更多主题颜色一键式操作简单便捷</h4>
	       <div class="title-theme"></div>
	    </div>
        <div class="width-flex">
            <h3 id="REGION_dv6">6.页面结构组成</h3>
            <h4>
               常规的页面构成主要分为 页面标题，表单筛选，表格列表数据，分页插件，空数据占位符
            </h4>
            <code showheader="true" minimized="false">

&lt;div class=&quot;form-info&quot;&gt;&lt;/div&gt;
&lt;div class=&quot;form-body&quot;&gt;&lt;/div&gt;
&lt;div class=&quot;table-body&quot;&gt;&lt;/div&gt;
&lt;div class=&quot;form-footer&quot;&gt;

    &lt;div class=&quot;paginationControl&quot;&gt;&lt;/div&gt;
&lt;/div&gt;
&lt;div class=&quot;norecordmsg fa hidden&quot;&gt;&lt;message key=&quot;global_msg.no_record_found&quot;&gt;&lt;/message&gt;&lt;/div&gt;

            </code>
        </div>
	</div>
</div>


<script type="text/javascript">

    var REGION = {
        goSection: function (event,sectionNum) {
            var target= RS.getEventTarget(event);
            var url = window.location.protocol + "//" + window.location.host + window.location.pathname + location.search + "#REGION_dv" + sectionNum;
            window.location.href = url;
            $(target).addClass('guides-active').siblings().removeClass('guides-active')
        },
        afterRenderData: function () {
            //var paraName = this.paraMap.get("paraName");
            var curRegion = this;
			RegionUtil.loadRegion(curRegion.find(".title-form"), "css/skin1/guide/main/form.rs");
			RegionUtil.loadRegion(curRegion.find(".title-table"), "css/skin1/guide/main/table.rs");
			RegionUtil.loadRegion(curRegion.find(".title-table1"), "css/skin1/guide/main/table1.rs");
			RegionUtil.loadRegion(curRegion.find(".title-table2"), "css/skin1/guide/main/table2.rs");
			var paraMap = new HashMap();
			paraMap.put("columns",1);
			paraMap.put("rows",1);
			paraMap.put("show",1);
			var loadPromise = RS.loadRegion($(".title-theme"),RS.appendParaMapToUrl("common/shared/theme/theme-box.rs",paraMap));
			loadPromise.done(function(iconRegion){
				$.getJSON('common/shared/theme/theme.json',function(themData){
					iconRegion.renderSystemEntry(themData);
				})
			})

        }
    };


    RS.ready(function () {
        var region = RS.newFormRegion("#REGION", null);
        region.afterRenderData = REGION.afterRenderData;
        region.renderRegion();
    })
</script>