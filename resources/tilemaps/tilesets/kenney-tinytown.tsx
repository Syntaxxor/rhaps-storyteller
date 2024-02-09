<?xml version="1.0" encoding="UTF-8"?>
<tileset version="1.10" tiledversion="1.10.2" name="kenney-tinytown" tilewidth="16" tileheight="16" tilecount="132" columns="12">
 <image source="kenney-tinytown.png" width="192" height="176"/>
 <tile id="2" probability="0.2"/>
 <tile id="43" probability="0.1"/>
 <wangsets>
  <wangset name="Grounds" type="corner" tile="-1">
   <wangcolor name="Grass" color="#ff0000" tile="-1" probability="1"/>
   <wangcolor name="Path" color="#00ff00" tile="-1" probability="1"/>
   <wangtile tileid="0" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="1" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="2" wangid="0,1,0,1,0,1,0,1"/>
   <wangtile tileid="12" wangid="0,1,0,2,0,1,0,1"/>
   <wangtile tileid="13" wangid="0,1,0,2,0,2,0,1"/>
   <wangtile tileid="14" wangid="0,1,0,1,0,2,0,1"/>
   <wangtile tileid="24" wangid="0,2,0,2,0,1,0,1"/>
   <wangtile tileid="25" wangid="0,2,0,2,0,2,0,2"/>
   <wangtile tileid="26" wangid="0,1,0,1,0,2,0,2"/>
   <wangtile tileid="36" wangid="0,2,0,1,0,1,0,1"/>
   <wangtile tileid="37" wangid="0,2,0,1,0,1,0,2"/>
   <wangtile tileid="38" wangid="0,1,0,1,0,1,0,2"/>
   <wangtile tileid="39" wangid="0,2,0,2,0,2,0,1"/>
   <wangtile tileid="40" wangid="0,1,0,2,0,2,0,2"/>
   <wangtile tileid="41" wangid="0,2,0,1,0,2,0,2"/>
   <wangtile tileid="42" wangid="0,2,0,2,0,1,0,2"/>
   <wangtile tileid="43" wangid="0,1,0,1,0,1,0,1"/>
  </wangset>
  <wangset name="Houses" type="edge" tile="-1">
   <wangcolor name="BrownHouse" color="#ff0000" tile="-1" probability="1"/>
   <wangcolor name="GreyHouse" color="#00ff00" tile="-1" probability="1"/>
   <wangtile tileid="72" wangid="0,0,1,0,0,0,0,0"/>
   <wangtile tileid="73" wangid="0,0,1,0,0,0,1,0"/>
   <wangtile tileid="75" wangid="0,0,0,0,0,0,1,0"/>
   <wangtile tileid="76" wangid="0,0,2,0,0,0,0,0"/>
   <wangtile tileid="77" wangid="0,0,2,0,0,0,2,0"/>
   <wangtile tileid="79" wangid="0,0,0,0,0,0,2,0"/>
  </wangset>
  <wangset name="Roofs" type="corner" tile="-1">
   <wangcolor name="GreyRoof" color="#ff0000" tile="-1" probability="1"/>
   <wangcolor name="RedRoof" color="#00ff00" tile="-1" probability="1"/>
   <wangcolor name="CastleRoof" color="#0000ff" tile="-1" probability="1"/>
   <wangtile tileid="48" wangid="0,0,0,1,0,0,0,0"/>
   <wangtile tileid="49" wangid="0,0,0,1,0,1,0,0"/>
   <wangtile tileid="50" wangid="0,0,0,0,0,1,0,0"/>
   <wangtile tileid="52" wangid="0,0,0,2,0,0,0,0"/>
   <wangtile tileid="53" wangid="0,0,0,2,0,2,0,0"/>
   <wangtile tileid="54" wangid="0,0,0,0,0,2,0,0"/>
   <wangtile tileid="60" wangid="0,1,0,0,0,0,0,0"/>
   <wangtile tileid="61" wangid="0,1,0,0,0,0,0,1"/>
   <wangtile tileid="62" wangid="0,0,0,0,0,0,0,1"/>
   <wangtile tileid="64" wangid="0,2,0,0,0,0,0,0"/>
   <wangtile tileid="65" wangid="0,2,0,0,0,0,0,2"/>
   <wangtile tileid="66" wangid="0,0,0,0,0,0,0,2"/>
   <wangtile tileid="96" wangid="0,0,0,3,0,0,0,0"/>
   <wangtile tileid="97" wangid="0,0,0,3,0,3,0,0"/>
   <wangtile tileid="98" wangid="0,0,0,0,0,3,0,0"/>
   <wangtile tileid="108" wangid="0,3,0,3,0,0,0,0"/>
   <wangtile tileid="109" wangid="0,3,0,3,0,3,0,3"/>
   <wangtile tileid="110" wangid="0,0,0,0,0,3,0,3"/>
   <wangtile tileid="120" wangid="0,3,0,0,0,0,0,0"/>
   <wangtile tileid="121" wangid="0,3,0,0,0,0,0,3"/>
   <wangtile tileid="122" wangid="0,0,0,0,0,0,0,3"/>
  </wangset>
  <wangset name="Fence" type="edge" tile="-1">
   <wangcolor name="" color="#ff0000" tile="-1" probability="1"/>
   <wangtile tileid="44" wangid="0,0,1,0,1,0,0,0"/>
   <wangtile tileid="45" wangid="0,0,1,0,0,0,1,0"/>
   <wangtile tileid="46" wangid="0,0,0,0,1,0,1,0"/>
   <wangtile tileid="47" wangid="0,0,0,0,1,0,0,0"/>
   <wangtile tileid="56" wangid="1,0,0,0,1,0,0,0"/>
   <wangtile tileid="58" wangid="1,0,0,0,1,0,0,0"/>
   <wangtile tileid="59" wangid="1,0,0,0,1,0,0,0"/>
   <wangtile tileid="68" wangid="1,0,1,0,0,0,0,0"/>
   <wangtile tileid="70" wangid="1,0,0,0,0,0,1,0"/>
   <wangtile tileid="71" wangid="1,0,0,0,0,0,0,0"/>
   <wangtile tileid="80" wangid="0,0,1,0,0,0,0,0"/>
   <wangtile tileid="81" wangid="0,0,1,0,0,0,1,0"/>
   <wangtile tileid="82" wangid="0,0,0,0,0,0,1,0"/>
  </wangset>
  <wangset name="Trees" type="edge" tile="-1">
   <wangcolor name="Green" color="#ff0000" tile="-1" probability="1"/>
   <wangcolor name="Yellow" color="#00ff00" tile="-1" probability="1"/>
   <wangtile tileid="6" wangid="0,0,1,0,1,0,0,0"/>
   <wangtile tileid="7" wangid="0,0,0,0,1,0,0,0"/>
   <wangtile tileid="8" wangid="0,0,0,0,1,0,1,0"/>
   <wangtile tileid="9" wangid="0,0,2,0,2,0,0,0"/>
   <wangtile tileid="10" wangid="0,0,0,0,2,0,0,0"/>
   <wangtile tileid="11" wangid="0,0,0,0,2,0,2,0"/>
   <wangtile tileid="18" wangid="0,0,1,0,0,0,0,0"/>
   <wangtile tileid="19" wangid="1,0,1,0,1,0,1,0"/>
   <wangtile tileid="20" wangid="0,0,0,0,0,0,1,0"/>
   <wangtile tileid="21" wangid="0,0,2,0,0,0,0,0"/>
   <wangtile tileid="22" wangid="2,0,2,0,2,0,2,0"/>
   <wangtile tileid="23" wangid="0,0,0,0,0,0,2,0"/>
   <wangtile tileid="30" wangid="1,0,1,0,0,0,0,0"/>
   <wangtile tileid="31" wangid="1,0,0,0,0,0,0,0"/>
   <wangtile tileid="32" wangid="1,0,0,0,0,0,1,0"/>
   <wangtile tileid="33" wangid="2,0,2,0,0,0,0,0"/>
   <wangtile tileid="34" wangid="2,0,0,0,0,0,0,0"/>
   <wangtile tileid="35" wangid="2,0,0,0,0,0,2,0"/>
  </wangset>
 </wangsets>
</tileset>
