function find(a:array of integer;x:integer):boolean;
begin
  for var i:=0 to a.length-1 do
    if(a[i]=x) then
    begin
      result:=true;
      exit;
    end;
    result:=false;
end;

begin
  var x:=arr(1,2,3,4,5);
  print(find(x,-1));
end.