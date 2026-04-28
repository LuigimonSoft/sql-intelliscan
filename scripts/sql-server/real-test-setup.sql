USE [master];
GO

IF OBJECT_ID(N'dbo.IntelliscanScalarSmokeItems', N'U') IS NOT NULL
BEGIN
    DROP TABLE dbo.IntelliscanScalarSmokeItems;
END;
GO

CREATE TABLE dbo.IntelliscanScalarSmokeItems
(
    Id INT IDENTITY(1,1) NOT NULL PRIMARY KEY,
    Name NVARCHAR(64) NOT NULL
);
GO

INSERT INTO dbo.IntelliscanScalarSmokeItems (Name)
VALUES
    (N'alpha'),
    (N'beta'),
    (N'gamma');
GO
