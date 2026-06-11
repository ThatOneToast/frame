// Generated typed Frame IR. Do not edit; regenerate with `frame build`.
// Source: src/app.frame
// Ownership: generated-only

import { defineFrameIrDocument } from '@frame/runtime-dom';

const ir = defineFrameIrDocument({
  "version": 1,
  "components": [
    {
      "name": "LLMDashboard",
      "props": [],
      "state": [],
      "slots": [],
      "nodes": [
        {
          "Element": {
            "kind": "screen",
            "semantic_kind": "screen",
            "render_kind": "div",
            "name": "AppShell",
            "style": {
              "Automatic": {
                "style": "AppShell",
                "source": {
                  "start": 232,
                  "end": 240
                }
              }
            },
            "attributes": [],
            "bindings": [],
            "events": [],
            "conditions": [],
            "children": [
              {
                "Element": {
                  "kind": "panel",
                  "semantic_kind": "panel",
                  "render_kind": "div",
                  "name": "Header",
                  "style": {
                    "Automatic": {
                      "style": "Header",
                      "source": {
                        "start": 255,
                        "end": 261
                      }
                    }
                  },
                  "attributes": [],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [
                    {
                      "Element": {
                        "kind": "row",
                        "semantic_kind": "row",
                        "render_kind": "div",
                        "name": "NavBar",
                        "style": {
                          "Automatic": {
                            "style": "NavBar",
                            "source": {
                              "start": 276,
                              "end": 282
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "Logo",
                              "style": {
                                "Automatic": {
                                  "style": "Logo",
                                  "source": {
                                    "start": 301,
                                    "end": 305
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "LLM"
                                    },
                                    "source": {
                                      "start": 308,
                                      "end": 330
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Ops"
                                    },
                                    "source": {
                                      "start": 331,
                                      "end": 353
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 285,
                                "end": 365
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "SearchBar",
                              "style": {
                                "Automatic": {
                                  "style": "SearchBar",
                                  "source": {
                                    "start": 380,
                                    "end": 389
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Search models, runs, or metrics..."
                                    },
                                    "source": {
                                      "start": 392,
                                      "end": 445
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 366,
                                "end": 457
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "StatusCluster",
                              "style": {
                                "Automatic": {
                                  "style": "StatusCluster",
                                  "source": {
                                    "start": 472,
                                    "end": 485
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "StatusBadge",
                                    "style": {
                                      "Automatic": {
                                        "style": "StatusBadge",
                                        "source": {
                                          "start": 505,
                                          "end": 516
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Text": {
                                          "value": {
                                            "Literal": "4 models online"
                                          },
                                          "source": {
                                            "start": 519,
                                            "end": 555
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 488,
                                      "end": 569
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "row",
                                    "semantic_kind": "row",
                                    "render_kind": "div",
                                    "name": "UserInfo",
                                    "style": {
                                      "Automatic": {
                                        "style": "UserInfo",
                                        "source": {
                                          "start": 586,
                                          "end": 594
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Text": {
                                          "value": {
                                            "Literal": "admin@local"
                                          },
                                          "source": {
                                            "start": 597,
                                            "end": 629
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 570,
                                      "end": 643
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 458,
                                "end": 655
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 264,
                          "end": 665
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 243,
                    "end": 673
                  }
                }
              },
              {
                "Element": {
                  "kind": "panel",
                  "semantic_kind": "panel",
                  "render_kind": "div",
                  "name": "SidebarNav",
                  "style": {
                    "Automatic": {
                      "style": "SidebarNav",
                      "source": {
                        "start": 686,
                        "end": 696
                      }
                    }
                  },
                  "attributes": [],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [
                    {
                      "Element": {
                        "kind": "stack",
                        "semantic_kind": "stack",
                        "render_kind": "div",
                        "name": "Navigation",
                        "style": {
                          "Automatic": {
                            "style": "Navigation",
                            "source": {
                              "start": 713,
                              "end": 723
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavSection",
                              "style": {
                                "Automatic": {
                                  "style": "NavSection",
                                  "source": {
                                    "start": 742,
                                    "end": 752
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "NAVIGATE"
                                    },
                                    "source": {
                                      "start": 755,
                                      "end": 782
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 726,
                                "end": 794
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavGroupDash",
                              "style": {
                                "Automatic": {
                                  "style": "NavGroupDash",
                                  "source": {
                                    "start": 811,
                                    "end": 823
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Dashboard"
                                    },
                                    "source": {
                                      "start": 826,
                                      "end": 854
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Models"
                                    },
                                    "source": {
                                      "start": 855,
                                      "end": 880
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Inference"
                                    },
                                    "source": {
                                      "start": 881,
                                      "end": 909
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Agents"
                                    },
                                    "source": {
                                      "start": 910,
                                      "end": 935
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 795,
                                "end": 947
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavSection",
                              "style": {
                                "Automatic": {
                                  "style": "NavSection",
                                  "source": {
                                    "start": 964,
                                    "end": 974
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "ANALYTICS"
                                    },
                                    "source": {
                                      "start": 977,
                                      "end": 1005
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 948,
                                "end": 1017
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavGroupAnalytics",
                              "style": {
                                "Automatic": {
                                  "style": "NavGroupAnalytics",
                                  "source": {
                                    "start": 1034,
                                    "end": 1051
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Benchmarks"
                                    },
                                    "source": {
                                      "start": 1054,
                                      "end": 1083
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Context"
                                    },
                                    "source": {
                                      "start": 1084,
                                      "end": 1110
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Prompts"
                                    },
                                    "source": {
                                      "start": 1111,
                                      "end": 1137
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1018,
                                "end": 1149
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavSection",
                              "style": {
                                "Automatic": {
                                  "style": "NavSection",
                                  "source": {
                                    "start": 1166,
                                    "end": 1176
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "SYSTEM"
                                    },
                                    "source": {
                                      "start": 1179,
                                      "end": 1204
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1150,
                                "end": 1216
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "stack",
                              "semantic_kind": "stack",
                              "render_kind": "div",
                              "name": "NavGroupSystem",
                              "style": {
                                "Automatic": {
                                  "style": "NavGroupSystem",
                                  "source": {
                                    "start": 1233,
                                    "end": 1247
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Logs"
                                    },
                                    "source": {
                                      "start": 1250,
                                      "end": 1273
                                    }
                                  }
                                },
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "Settings"
                                    },
                                    "source": {
                                      "start": 1274,
                                      "end": 1301
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1217,
                                "end": 1313
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 699,
                          "end": 1323
                        }
                      }
                    },
                    {
                      "Element": {
                        "kind": "stack",
                        "semantic_kind": "stack",
                        "render_kind": "div",
                        "name": "SidebarFooter",
                        "style": {
                          "Automatic": {
                            "style": "SidebarFooter",
                            "source": {
                              "start": 1338,
                              "end": 1351
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "UserStatus",
                              "style": {
                                "Automatic": {
                                  "style": "UserStatus",
                                  "source": {
                                    "start": 1368,
                                    "end": 1378
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Text": {
                                    "value": {
                                      "Literal": "admin@local"
                                    },
                                    "source": {
                                      "start": 1381,
                                      "end": 1411
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1354,
                                "end": 1423
                              }
                            }
                          },
                          {
                            "Text": {
                              "value": {
                                "Literal": "v0.4.2"
                              },
                              "source": {
                                "start": 1424,
                                "end": 1447
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 1324,
                          "end": 1457
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 674,
                    "end": 1465
                  }
                }
              },
              {
                "Element": {
                  "kind": "panel",
                  "semantic_kind": "panel",
                  "render_kind": "div",
                  "name": "MainContent",
                  "style": {
                    "Automatic": {
                      "style": "MainContent",
                      "source": {
                        "start": 1478,
                        "end": 1489
                      }
                    }
                  },
                  "attributes": [],
                  "bindings": [],
                  "events": [],
                  "conditions": [],
                  "children": [
                    {
                      "Element": {
                        "kind": "stack",
                        "semantic_kind": "stack",
                        "render_kind": "div",
                        "name": "DashboardContent",
                        "style": {
                          "Automatic": {
                            "style": "DashboardContent",
                            "source": {
                              "start": 1506,
                              "end": 1522
                            }
                          }
                        },
                        "attributes": [],
                        "bindings": [],
                        "events": [],
                        "conditions": [],
                        "children": [
                          {
                            "Element": {
                              "kind": "row",
                              "semantic_kind": "row",
                              "render_kind": "div",
                              "name": "MetricRow",
                              "style": {
                                "Automatic": {
                                  "style": "MetricRow",
                                  "source": {
                                    "start": 1539,
                                    "end": 1548
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardModel",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardModel",
                                        "source": {
                                          "start": 1568,
                                          "end": 1583
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconModel",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconModel",
                                              "source": {
                                                "start": 1606,
                                                "end": 1621
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "MODEL"
                                                },
                                                "source": {
                                                  "start": 1624,
                                                  "end": 1652
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1586,
                                            "end": 1668
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentModel",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentModel",
                                              "source": {
                                                "start": 1689,
                                                "end": 1707
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Active Model"
                                                },
                                                "source": {
                                                  "start": 1710,
                                                  "end": 1745
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Qwen3.6 Coder 27B"
                                                },
                                                "source": {
                                                  "start": 1746,
                                                  "end": 1786
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "MLX · 27B params"
                                                },
                                                "source": {
                                                  "start": 1787,
                                                  "end": 1831
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1669,
                                            "end": 1847
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaModel",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaModel",
                                              "source": {
                                                "start": 1867,
                                                "end": 1877
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Healthy"
                                                },
                                                "source": {
                                                  "start": 1880,
                                                  "end": 1910
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1848,
                                            "end": 1926
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 1551,
                                      "end": 1940
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardTps",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardTps",
                                        "source": {
                                          "start": 1958,
                                          "end": 1971
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconTps",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconTps",
                                              "source": {
                                                "start": 1994,
                                                "end": 2007
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "TPS"
                                                },
                                                "source": {
                                                  "start": 2010,
                                                  "end": 2036
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 1974,
                                            "end": 2052
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentTps",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentTps",
                                              "source": {
                                                "start": 2073,
                                                "end": 2089
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Tokens/sec"
                                                },
                                                "source": {
                                                  "start": 2092,
                                                  "end": 2125
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "42.5"
                                                },
                                                "source": {
                                                  "start": 2126,
                                                  "end": 2153
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "peak 51.2"
                                                },
                                                "source": {
                                                  "start": 2154,
                                                  "end": 2186
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2053,
                                            "end": 2202
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaTps",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaTps",
                                              "source": {
                                                "start": 2222,
                                                "end": 2230
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "+8.3%"
                                                },
                                                "source": {
                                                  "start": 2233,
                                                  "end": 2261
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2203,
                                            "end": 2277
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 1941,
                                      "end": 2291
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardCtx",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardCtx",
                                        "source": {
                                          "start": 2309,
                                          "end": 2322
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconCtx",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconCtx",
                                              "source": {
                                                "start": 2345,
                                                "end": 2358
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "CTX"
                                                },
                                                "source": {
                                                  "start": 2361,
                                                  "end": 2387
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2325,
                                            "end": 2403
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentCtx",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentCtx",
                                              "source": {
                                                "start": 2424,
                                                "end": 2440
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Context Window"
                                                },
                                                "source": {
                                                  "start": 2443,
                                                  "end": 2480
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "18,432 / 32,768"
                                                },
                                                "source": {
                                                  "start": 2481,
                                                  "end": 2519
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "56.3% utilized"
                                                },
                                                "source": {
                                                  "start": 2520,
                                                  "end": 2557
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2404,
                                            "end": 2573
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaCtx",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaCtx",
                                              "source": {
                                                "start": 2593,
                                                "end": 2601
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "normal"
                                                },
                                                "source": {
                                                  "start": 2604,
                                                  "end": 2633
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2574,
                                            "end": 2649
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 2292,
                                      "end": 2663
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "MetricCardVram",
                                    "style": {
                                      "Automatic": {
                                        "style": "MetricCardVram",
                                        "source": {
                                          "start": 2681,
                                          "end": 2695
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricIconVram",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricIconVram",
                                              "source": {
                                                "start": 2718,
                                                "end": 2732
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "VRAM"
                                                },
                                                "source": {
                                                  "start": 2735,
                                                  "end": 2762
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2698,
                                            "end": 2778
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "MetricContentVram",
                                          "style": {
                                            "Automatic": {
                                              "style": "MetricContentVram",
                                              "source": {
                                                "start": 2799,
                                                "end": 2816
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "GPU Memory"
                                                },
                                                "source": {
                                                  "start": 2819,
                                                  "end": 2852
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "14.2 GB / 24.0 GB"
                                                },
                                                "source": {
                                                  "start": 2853,
                                                  "end": 2893
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "NVIDIA RTX 4090"
                                                },
                                                "source": {
                                                  "start": 2894,
                                                  "end": 2932
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2779,
                                            "end": 2948
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "card",
                                          "semantic_kind": "card",
                                          "render_kind": "div",
                                          "name": "DeltaVram",
                                          "style": {
                                            "Automatic": {
                                              "style": "DeltaVram",
                                              "source": {
                                                "start": 2968,
                                                "end": 2977
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "59.2%"
                                                },
                                                "source": {
                                                  "start": 2980,
                                                  "end": 3008
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 2949,
                                            "end": 3024
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 2664,
                                      "end": 3038
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 1525,
                                "end": 3050
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "grid",
                              "semantic_kind": "grid",
                              "render_kind": "div",
                              "name": "PerformanceGrid",
                              "style": {
                                "Automatic": {
                                  "style": "PerformanceGrid",
                                  "source": {
                                    "start": 3066,
                                    "end": 3081
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "PerformanceChart",
                                    "style": {
                                      "Automatic": {
                                        "style": "PerformanceChart",
                                        "source": {
                                          "start": 3101,
                                          "end": 3117
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "ChartHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "ChartHeader",
                                              "source": {
                                                "start": 3140,
                                                "end": 3151
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ChartTitle",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartTitle",
                                                    "source": {
                                                      "start": 3174,
                                                      "end": 3184
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Inference Performance"
                                                      },
                                                      "source": {
                                                        "start": 3187,
                                                        "end": 3233
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "ChartRange",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ChartRange",
                                                          "source": {
                                                            "start": 3257,
                                                            "end": 3267
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "60m"
                                                            },
                                                            "source": {
                                                              "start": 3270,
                                                              "end": 3300
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 3234,
                                                        "end": 3320
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 3154,
                                                  "end": 3338
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ChartLegend",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartLegend",
                                                    "source": {
                                                      "start": 3359,
                                                      "end": 3370
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "row",
                                                      "semantic_kind": "row",
                                                      "render_kind": "div",
                                                      "name": "LegendItemA",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "LegendItemA",
                                                          "source": {
                                                            "start": 3395,
                                                            "end": 3406
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.6 Coder 27B"
                                                            },
                                                            "source": {
                                                              "start": 3409,
                                                              "end": 3453
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 3373,
                                                        "end": 3473
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "row",
                                                      "semantic_kind": "row",
                                                      "render_kind": "div",
                                                      "name": "LegendItemB",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "LegendItemB",
                                                          "source": {
                                                            "start": 3496,
                                                            "end": 3507
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.5 0.8B Draft"
                                                            },
                                                            "source": {
                                                              "start": 3510,
                                                              "end": 3555
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 3474,
                                                        "end": 3575
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 3339,
                                                  "end": 3593
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 3120,
                                            "end": 3609
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "ChartArea",
                                          "style": {
                                            "Automatic": {
                                              "style": "ChartArea",
                                              "source": {
                                                "start": 3630,
                                                "end": 3639
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ChartBars",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartBars",
                                                    "source": {
                                                      "start": 3662,
                                                      "end": 3671
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [],
                                                "source": {
                                                  "start": 3642,
                                                  "end": 3691
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "card",
                                                "semantic_kind": "card",
                                                "render_kind": "div",
                                                "name": "ChartBar1",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartBar1",
                                                    "source": {
                                                      "start": 3713,
                                                      "end": 3722
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [],
                                                "source": {
                                                  "start": 3692,
                                                  "end": 3742
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "card",
                                                "semantic_kind": "card",
                                                "render_kind": "div",
                                                "name": "ChartBar2",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartBar2",
                                                    "source": {
                                                      "start": 3764,
                                                      "end": 3773
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [],
                                                "source": {
                                                  "start": 3743,
                                                  "end": 3793
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "card",
                                                "semantic_kind": "card",
                                                "render_kind": "div",
                                                "name": "ChartBar3",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartBar3",
                                                    "source": {
                                                      "start": 3815,
                                                      "end": 3824
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [],
                                                "source": {
                                                  "start": 3794,
                                                  "end": 3844
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "card",
                                                "semantic_kind": "card",
                                                "render_kind": "div",
                                                "name": "ChartBar4",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartBar4",
                                                    "source": {
                                                      "start": 3866,
                                                      "end": 3875
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [],
                                                "source": {
                                                  "start": 3845,
                                                  "end": 3895
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "card",
                                                "semantic_kind": "card",
                                                "render_kind": "div",
                                                "name": "ChartBar5",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartBar5",
                                                    "source": {
                                                      "start": 3917,
                                                      "end": 3926
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [],
                                                "source": {
                                                  "start": 3896,
                                                  "end": 3946
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ChartAxis",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ChartAxis",
                                                    "source": {
                                                      "start": 3967,
                                                      "end": 3976
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:00"
                                                      },
                                                      "source": {
                                                        "start": 3979,
                                                        "end": 4009
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:15"
                                                      },
                                                      "source": {
                                                        "start": 4010,
                                                        "end": 4040
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:30"
                                                      },
                                                      "source": {
                                                        "start": 4041,
                                                        "end": 4071
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:45"
                                                      },
                                                      "source": {
                                                        "start": 4072,
                                                        "end": 4102
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "14:55"
                                                      },
                                                      "source": {
                                                        "start": 4103,
                                                        "end": 4133
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 3947,
                                                  "end": 4151
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 3610,
                                            "end": 4167
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 3084,
                                      "end": 4181
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "RunPrompt",
                                    "style": {
                                      "Automatic": {
                                        "style": "RunPrompt",
                                        "source": {
                                          "start": 4199,
                                          "end": 4208
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "PromptHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "PromptHeader",
                                              "source": {
                                                "start": 4231,
                                                "end": 4243
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Quick Inference Test"
                                                },
                                                "source": {
                                                  "start": 4246,
                                                  "end": 4289
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 4211,
                                            "end": 4305
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "PromptContent",
                                          "style": {
                                            "Automatic": {
                                              "style": "PromptContent",
                                              "source": {
                                                "start": 4326,
                                                "end": 4339
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Element": {
                                                "kind": "stack",
                                                "semantic_kind": "stack",
                                                "render_kind": "div",
                                                "name": "PromptInput",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "PromptInput",
                                                    "source": {
                                                      "start": 4364,
                                                      "end": 4375
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Enter a test prompt..."
                                                      },
                                                      "source": {
                                                        "start": 4378,
                                                        "end": 4425
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4342,
                                                  "end": 4443
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "action",
                                                "semantic_kind": "action",
                                                "render_kind": "button",
                                                "name": "RunTest",
                                                "style": {
                                                  "Explicit": {
                                                    "style": "PrimaryButton",
                                                    "source": {
                                                      "start": 4475,
                                                      "end": 4488
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [
                                                  {
                                                    "event": "press",
                                                    "modifiers": [],
                                                    "handler": "runTest",
                                                    "source": {
                                                      "start": 4530,
                                                      "end": 4565
                                                    }
                                                  }
                                                ],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Run Inference"
                                                      },
                                                      "source": {
                                                        "start": 4491,
                                                        "end": 4529
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4444,
                                                  "end": 4583
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 4306,
                                            "end": 4599
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "PromptMeta",
                                          "style": {
                                            "Automatic": {
                                              "style": "PromptMeta",
                                              "source": {
                                                "start": 4620,
                                                "end": 4630
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Model: Qwen3.6 Coder 27B"
                                                },
                                                "source": {
                                                  "start": 4633,
                                                  "end": 4680
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Provider: Local MLX"
                                                },
                                                "source": {
                                                  "start": 4681,
                                                  "end": 4723
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 4600,
                                            "end": 4739
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 4182,
                                      "end": 4753
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 3051,
                                "end": 4765
                              }
                            }
                          },
                          {
                            "Element": {
                              "kind": "grid",
                              "semantic_kind": "grid",
                              "render_kind": "div",
                              "name": "DashboardGrid",
                              "style": {
                                "Automatic": {
                                  "style": "DashboardGrid",
                                  "source": {
                                    "start": 4781,
                                    "end": 4794
                                  }
                                }
                              },
                              "attributes": [],
                              "bindings": [],
                              "events": [],
                              "conditions": [],
                              "children": [
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "InferenceRuns",
                                    "style": {
                                      "Automatic": {
                                        "style": "InferenceRuns",
                                        "source": {
                                          "start": 4814,
                                          "end": 4827
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "RunsHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "RunsHeader",
                                              "source": {
                                                "start": 4850,
                                                "end": 4860
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunsTitleRow",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunsTitleRow",
                                                    "source": {
                                                      "start": 4883,
                                                      "end": 4895
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Recent Inference Runs"
                                                      },
                                                      "source": {
                                                        "start": 4898,
                                                        "end": 4944
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4863,
                                                  "end": 4962
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunTabs",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunTabs",
                                                    "source": {
                                                      "start": 4983,
                                                      "end": 4990
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "action",
                                                      "semantic_kind": "action",
                                                      "render_kind": "button",
                                                      "name": "TabAll",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "TabAll",
                                                          "source": {
                                                            "start": 5018,
                                                            "end": 5024
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [
                                                        {
                                                          "event": "press",
                                                          "modifiers": [],
                                                          "handler": "filterRuns",
                                                          "source": {
                                                            "start": 5058,
                                                            "end": 5098
                                                          }
                                                        }
                                                      ],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "All"
                                                            },
                                                            "source": {
                                                              "start": 5027,
                                                              "end": 5057
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 4993,
                                                        "end": 5118
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "action",
                                                      "semantic_kind": "action",
                                                      "render_kind": "button",
                                                      "name": "TabLocal",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "TabLocal",
                                                          "source": {
                                                            "start": 5144,
                                                            "end": 5152
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [
                                                        {
                                                          "event": "press",
                                                          "modifiers": [],
                                                          "handler": "filterRuns",
                                                          "source": {
                                                            "start": 5188,
                                                            "end": 5228
                                                          }
                                                        }
                                                      ],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Local"
                                                            },
                                                            "source": {
                                                              "start": 5155,
                                                              "end": 5187
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 5119,
                                                        "end": 5248
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "action",
                                                      "semantic_kind": "action",
                                                      "render_kind": "button",
                                                      "name": "TabRemote",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "TabRemote",
                                                          "source": {
                                                            "start": 5274,
                                                            "end": 5283
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [
                                                        {
                                                          "event": "press",
                                                          "modifiers": [],
                                                          "handler": "filterRuns",
                                                          "source": {
                                                            "start": 5320,
                                                            "end": 5360
                                                          }
                                                        }
                                                      ],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Remote"
                                                            },
                                                            "source": {
                                                              "start": 5286,
                                                              "end": 5319
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 5249,
                                                        "end": 5380
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 4963,
                                                  "end": 5398
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 4830,
                                            "end": 5414
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "RunsTableHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "RunsTableHeader",
                                              "source": {
                                                "start": 5435,
                                                "end": 5450
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunColHeaders",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunColHeaders",
                                                    "source": {
                                                      "start": 5473,
                                                      "end": 5486
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Model"
                                                      },
                                                      "source": {
                                                        "start": 5489,
                                                        "end": 5519
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Provider"
                                                      },
                                                      "source": {
                                                        "start": 5520,
                                                        "end": 5553
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Speed"
                                                      },
                                                      "source": {
                                                        "start": 5554,
                                                        "end": 5584
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Latency"
                                                      },
                                                      "source": {
                                                        "start": 5585,
                                                        "end": 5617
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Tokens"
                                                      },
                                                      "source": {
                                                        "start": 5618,
                                                        "end": 5649
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Cost"
                                                      },
                                                      "source": {
                                                        "start": 5650,
                                                        "end": 5679
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 5453,
                                                  "end": 5697
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 5415,
                                            "end": 5713
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "RunsTable",
                                          "style": {
                                            "Automatic": {
                                              "style": "RunsTable",
                                              "source": {
                                                "start": 5734,
                                                "end": 5743
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow1",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow1",
                                                    "source": {
                                                      "start": 5766,
                                                      "end": 5773
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel1",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel1",
                                                          "source": {
                                                            "start": 5800,
                                                            "end": 5809
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.6 Coder 27B"
                                                            },
                                                            "source": {
                                                              "start": 5812,
                                                              "end": 5856
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "MLX local"
                                                            },
                                                            "source": {
                                                              "start": 5857,
                                                              "end": 5893
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 5776,
                                                        "end": 5913
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Local"
                                                      },
                                                      "source": {
                                                        "start": 5914,
                                                        "end": 5944
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "42.5 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 5945,
                                                        "end": 5980
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "23.5ms"
                                                      },
                                                      "source": {
                                                        "start": 5981,
                                                        "end": 6012
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "18,432"
                                                      },
                                                      "source": {
                                                        "start": 6013,
                                                        "end": 6044
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0000"
                                                      },
                                                      "source": {
                                                        "start": 6045,
                                                        "end": 6077
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 5746,
                                                  "end": 6095
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow2",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow2",
                                                    "source": {
                                                      "start": 6116,
                                                      "end": 6123
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel2",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel2",
                                                          "source": {
                                                            "start": 6150,
                                                            "end": 6159
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Gemma 3 27B"
                                                            },
                                                            "source": {
                                                              "start": 6162,
                                                              "end": 6200
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Ollama local"
                                                            },
                                                            "source": {
                                                              "start": 6201,
                                                              "end": 6240
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 6126,
                                                        "end": 6260
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Local"
                                                      },
                                                      "source": {
                                                        "start": 6261,
                                                        "end": 6291
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "35.8 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 6292,
                                                        "end": 6327
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "27.9ms"
                                                      },
                                                      "source": {
                                                        "start": 6328,
                                                        "end": 6359
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "12,288"
                                                      },
                                                      "source": {
                                                        "start": 6360,
                                                        "end": 6391
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0000"
                                                      },
                                                      "source": {
                                                        "start": 6392,
                                                        "end": 6424
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 6096,
                                                  "end": 6442
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow3",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow3",
                                                    "source": {
                                                      "start": 6463,
                                                      "end": 6470
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel3",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel3",
                                                          "source": {
                                                            "start": 6497,
                                                            "end": 6506
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Llama 3.1 8B"
                                                            },
                                                            "source": {
                                                              "start": 6509,
                                                              "end": 6548
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Together API"
                                                            },
                                                            "source": {
                                                              "start": 6549,
                                                              "end": 6588
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 6473,
                                                        "end": 6608
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Remote"
                                                      },
                                                      "source": {
                                                        "start": 6609,
                                                        "end": 6640
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "28.4 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 6641,
                                                        "end": 6676
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "35.2ms"
                                                      },
                                                      "source": {
                                                        "start": 6677,
                                                        "end": 6708
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "45,056"
                                                      },
                                                      "source": {
                                                        "start": 6709,
                                                        "end": 6740
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0042"
                                                      },
                                                      "source": {
                                                        "start": 6741,
                                                        "end": 6773
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 6443,
                                                  "end": 6791
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow4",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow4",
                                                    "source": {
                                                      "start": 6812,
                                                      "end": 6819
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel4",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel4",
                                                          "source": {
                                                            "start": 6846,
                                                            "end": 6855
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "DeepSeek Coder 6.7B"
                                                            },
                                                            "source": {
                                                              "start": 6858,
                                                              "end": 6904
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Local GPU"
                                                            },
                                                            "source": {
                                                              "start": 6905,
                                                              "end": 6941
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 6822,
                                                        "end": 6961
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Local"
                                                      },
                                                      "source": {
                                                        "start": 6962,
                                                        "end": 6992
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "31.2 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 6993,
                                                        "end": 7028
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "19.8ms"
                                                      },
                                                      "source": {
                                                        "start": 7029,
                                                        "end": 7060
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "8,192"
                                                      },
                                                      "source": {
                                                        "start": 7061,
                                                        "end": 7091
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0000"
                                                      },
                                                      "source": {
                                                        "start": 7092,
                                                        "end": 7124
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 6792,
                                                  "end": 7142
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "RunRow5",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "RunRow5",
                                                    "source": {
                                                      "start": 7163,
                                                      "end": 7170
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "RunModel5",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RunModel5",
                                                          "source": {
                                                            "start": 7197,
                                                            "end": 7206
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Phi 3.5 Mini"
                                                            },
                                                            "source": {
                                                              "start": 7209,
                                                              "end": 7248
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Azure endpoint"
                                                            },
                                                            "source": {
                                                              "start": 7249,
                                                              "end": 7290
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 7173,
                                                        "end": 7310
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "Remote"
                                                      },
                                                      "source": {
                                                        "start": 7311,
                                                        "end": 7342
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "52.1 tok/s"
                                                      },
                                                      "source": {
                                                        "start": 7343,
                                                        "end": 7378
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "12.4ms"
                                                      },
                                                      "source": {
                                                        "start": 7379,
                                                        "end": 7410
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "4,096"
                                                      },
                                                      "source": {
                                                        "start": 7411,
                                                        "end": 7441
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "$0.0001"
                                                      },
                                                      "source": {
                                                        "start": 7442,
                                                        "end": 7474
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 7143,
                                                  "end": 7492
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 5714,
                                            "end": 7508
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 4797,
                                      "end": 7522
                                    }
                                  }
                                },
                                {
                                  "Element": {
                                    "kind": "card",
                                    "semantic_kind": "card",
                                    "render_kind": "div",
                                    "name": "TopModels",
                                    "style": {
                                      "Automatic": {
                                        "style": "TopModels",
                                        "source": {
                                          "start": 7540,
                                          "end": 7549
                                        }
                                      }
                                    },
                                    "attributes": [],
                                    "bindings": [],
                                    "events": [],
                                    "conditions": [],
                                    "children": [
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "TopModelsHeader",
                                          "style": {
                                            "Automatic": {
                                              "style": "TopModelsHeader",
                                              "source": {
                                                "start": 7572,
                                                "end": 7587
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Top Models"
                                                },
                                                "source": {
                                                  "start": 7590,
                                                  "end": 7623
                                                }
                                              }
                                            },
                                            {
                                              "Text": {
                                                "value": {
                                                  "Literal": "Ranked by throughput"
                                                },
                                                "source": {
                                                  "start": 7624,
                                                  "end": 7667
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 7552,
                                            "end": 7683
                                          }
                                        }
                                      },
                                      {
                                        "Element": {
                                          "kind": "stack",
                                          "semantic_kind": "stack",
                                          "render_kind": "div",
                                          "name": "ModelsList",
                                          "style": {
                                            "Automatic": {
                                              "style": "ModelsList",
                                              "source": {
                                                "start": 7704,
                                                "end": 7714
                                              }
                                            }
                                          },
                                          "attributes": [],
                                          "bindings": [],
                                          "events": [],
                                          "conditions": [],
                                          "children": [
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow1",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow1",
                                                    "source": {
                                                      "start": 7737,
                                                      "end": 7746
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge1",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge1",
                                                          "source": {
                                                            "start": 7772,
                                                            "end": 7782
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "1"
                                                            },
                                                            "source": {
                                                              "start": 7785,
                                                              "end": 7813
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 7749,
                                                        "end": 7833
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo1",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo1",
                                                          "source": {
                                                            "start": 7858,
                                                            "end": 7868
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Qwen3.6 Coder 27B"
                                                            },
                                                            "source": {
                                                              "start": 7871,
                                                              "end": 7915
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "MLX · 27B"
                                                            },
                                                            "source": {
                                                              "start": 7916,
                                                              "end": 7957
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 7834,
                                                        "end": 7977
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "+11.2%"
                                                      },
                                                      "source": {
                                                        "start": 7978,
                                                        "end": 8009
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 7717,
                                                  "end": 8027
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow2",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow2",
                                                    "source": {
                                                      "start": 8048,
                                                      "end": 8057
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge2",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge2",
                                                          "source": {
                                                            "start": 8083,
                                                            "end": 8093
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "2"
                                                            },
                                                            "source": {
                                                              "start": 8096,
                                                              "end": 8124
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8060,
                                                        "end": 8144
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo2",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo2",
                                                          "source": {
                                                            "start": 8169,
                                                            "end": 8179
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Gemma 3 27B"
                                                            },
                                                            "source": {
                                                              "start": 8182,
                                                              "end": 8220
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Ollama · 27B"
                                                            },
                                                            "source": {
                                                              "start": 8221,
                                                              "end": 8265
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8145,
                                                        "end": 8285
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "-0.8%"
                                                      },
                                                      "source": {
                                                        "start": 8286,
                                                        "end": 8316
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 8028,
                                                  "end": 8334
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow3",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow3",
                                                    "source": {
                                                      "start": 8355,
                                                      "end": 8364
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge3",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge3",
                                                          "source": {
                                                            "start": 8390,
                                                            "end": 8400
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "3"
                                                            },
                                                            "source": {
                                                              "start": 8403,
                                                              "end": 8431
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8367,
                                                        "end": 8451
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo3",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo3",
                                                          "source": {
                                                            "start": 8476,
                                                            "end": 8486
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Llama 3.1 8B"
                                                            },
                                                            "source": {
                                                              "start": 8489,
                                                              "end": 8528
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Together · 8B"
                                                            },
                                                            "source": {
                                                              "start": 8529,
                                                              "end": 8574
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8452,
                                                        "end": 8594
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "+1.8%"
                                                      },
                                                      "source": {
                                                        "start": 8595,
                                                        "end": 8625
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 8335,
                                                  "end": 8643
                                                }
                                              }
                                            },
                                            {
                                              "Element": {
                                                "kind": "row",
                                                "semantic_kind": "row",
                                                "render_kind": "div",
                                                "name": "ModelRow4",
                                                "style": {
                                                  "Automatic": {
                                                    "style": "ModelRow4",
                                                    "source": {
                                                      "start": 8664,
                                                      "end": 8673
                                                    }
                                                  }
                                                },
                                                "attributes": [],
                                                "bindings": [],
                                                "events": [],
                                                "conditions": [],
                                                "children": [
                                                  {
                                                    "Element": {
                                                      "kind": "card",
                                                      "semantic_kind": "card",
                                                      "render_kind": "div",
                                                      "name": "RankBadge4",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "RankBadge4",
                                                          "source": {
                                                            "start": 8699,
                                                            "end": 8709
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "4"
                                                            },
                                                            "source": {
                                                              "start": 8712,
                                                              "end": 8740
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8676,
                                                        "end": 8760
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Element": {
                                                      "kind": "stack",
                                                      "semantic_kind": "stack",
                                                      "render_kind": "div",
                                                      "name": "ModelInfo4",
                                                      "style": {
                                                        "Automatic": {
                                                          "style": "ModelInfo4",
                                                          "source": {
                                                            "start": 8785,
                                                            "end": 8795
                                                          }
                                                        }
                                                      },
                                                      "attributes": [],
                                                      "bindings": [],
                                                      "events": [],
                                                      "conditions": [],
                                                      "children": [
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Phi 3.5 Mini"
                                                            },
                                                            "source": {
                                                              "start": 8798,
                                                              "end": 8837
                                                            }
                                                          }
                                                        },
                                                        {
                                                          "Text": {
                                                            "value": {
                                                              "Literal": "Azure · 3.8B"
                                                            },
                                                            "source": {
                                                              "start": 8838,
                                                              "end": 8882
                                                            }
                                                          }
                                                        }
                                                      ],
                                                      "source": {
                                                        "start": 8761,
                                                        "end": 8902
                                                      }
                                                    }
                                                  },
                                                  {
                                                    "Text": {
                                                      "value": {
                                                        "Literal": "+2.6%"
                                                      },
                                                      "source": {
                                                        "start": 8903,
                                                        "end": 8933
                                                      }
                                                    }
                                                  }
                                                ],
                                                "source": {
                                                  "start": 8644,
                                                  "end": 8951
                                                }
                                              }
                                            }
                                          ],
                                          "source": {
                                            "start": 7684,
                                            "end": 8967
                                          }
                                        }
                                      }
                                    ],
                                    "source": {
                                      "start": 7523,
                                      "end": 8981
                                    }
                                  }
                                }
                              ],
                              "source": {
                                "start": 4766,
                                "end": 8993
                              }
                            }
                          }
                        ],
                        "source": {
                          "start": 1492,
                          "end": 9003
                        }
                      }
                    }
                  ],
                  "source": {
                    "start": 1466,
                    "end": 9011
                  }
                }
              }
            ],
            "source": {
              "start": 221,
              "end": 9017
            }
          }
        }
      ],
      "capabilities": [
        "EventBinding"
      ],
      "source": {
        "start": 28,
        "end": 9023
      }
    }
  ]
} as const);

export default ir;
